use zkstd::common::{FftField, SigUtils};

use crate::{
    db::Db,
    domain::{Transaction, UserData},
    merkle_tree::{MerkleProof, SparseMerkleTree},
    poseidon::FieldHasher,
    proof::Proof,
};
#[cfg(test)]
use red_jubjub::PublicKey;

#[derive(Debug, PartialEq, Default)]
pub(crate) struct Batch<F: FftField> {
    transactions: Vec<Transaction>,
    roots: Vec<(F, F)>,
}

impl<F: FftField> Batch<F> {
    pub fn transactions(&self) -> impl Iterator<Item = &Transaction> {
        self.transactions.iter()
    }

    pub fn intermediate_roots(&self) -> Vec<(F, F)> {
        self.roots.clone()
    }

    pub fn border_roots(&self) -> (F, F) {
        (self.first_root(), self.final_root())
    }

    pub(crate) fn first_root(&self) -> F {
        self.roots
            .first()
            .expect("Batch size should be greater than zero")
            .0
    }

    pub(crate) fn final_root(&self) -> F {
        self.roots
            .last()
            .expect("Batch size should be greater than zero")
            .1
    }
}

#[derive(Default)]
pub(crate) struct RollupOperator<
    F: FftField,
    H: FieldHasher<F, 2>,
    const N: usize,
    const BATCH_SIZE: usize,
> {
    state_merkle: SparseMerkleTree<F, H, N>,
    db: Db,
    transactions: Vec<(Transaction, (F, F))>,
    index_counter: u64,
    hasher: H,
}

impl<F: FftField, H: FieldHasher<F, 2>, const N: usize, const BATCH_SIZE: usize>
    RollupOperator<F, H, N, BATCH_SIZE>
{
    // const BATCH_SIZE: usize = 2;

    pub fn new(hasher: H) -> Self {
        Self {
            state_merkle: SparseMerkleTree::new_empty(&hasher, &[0; 64])
                .expect("Failed to create state merkle tree"),
            hasher,
            ..Default::default()
        }
    }

    pub fn execute_transaction(
        &mut self,
        transaction: Transaction,
    ) -> Option<(Proof<F, H, N, BATCH_SIZE>, Batch<F>)> {
        let Transaction(signature, transaction_data) = transaction;
        let prev_root = self.state_root();
        let sender = self.db.get_mut(&transaction_data.sender_address);

        let sender_index = sender.index;

        self.state_merkle
            .generate_membership_proof(sender_index)
            .check_membership(
                &self.state_merkle.root(),
                &sender.to_field_element(),
                &self.hasher,
            )
            .expect("Sender is not presented in the state");

        assert!(transaction_data
            .sender_address
            .validate(&transaction_data.to_bytes(), signature));

        assert!(sender.balance >= transaction_data.amount);

        assert!(sender.balance >= transaction_data.amount);
        sender.balance -= transaction_data.amount;
        self.state_merkle
            .update(sender_index, sender.to_field_element(), &self.hasher)
            .expect("Failed to update balance");

        let receiver = self.db.get_mut(&transaction_data.receiver_address);

        let receiver_index = receiver.index;

        self.state_merkle
            .generate_membership_proof(receiver_index)
            .check_membership(
                &self.state_merkle.root(),
                &receiver.to_field_element(),
                &self.hasher,
            )
            .expect("Sender is not presented in the state");

        receiver.balance += transaction_data.amount;

        self.state_merkle
            .update(receiver_index, receiver.to_field_element(), &self.hasher)
            .expect("Failed to update balance");

        self.transactions
            .push((transaction, (prev_root, self.state_merkle.root())));

        if self.transactions.len() >= BATCH_SIZE {
            Some(self.process_batch())
        } else {
            None
        }
    }

    pub fn process_batch(&mut self) -> (Proof<F, H, N, BATCH_SIZE>, Batch<F>) {
        let batch = self.create_batch();
        let batch_leaves: Vec<F> = batch.transactions().map(|t| t.to_field_element()).collect();
        let batch_tree = SparseMerkleTree::<F, H, BATCH_SIZE>::new_sequential(
            &batch_leaves,
            &self.hasher,
            &[0; 64],
        )
        .expect("Failed to create batch merkle tree");

        let t_merkle_proofs: Vec<MerkleProof<F, H, BATCH_SIZE>> = (0..batch.transactions.len())
            .map(|index| batch_tree.generate_membership_proof(index as u64))
            .collect();
        let state_roots = batch.intermediate_roots();

        let sender_receiver_in_state_merkle_proofs = vec![];
        (
            self.create_proof(
                batch_tree,
                t_merkle_proofs,
                sender_receiver_in_state_merkle_proofs,
                state_roots,
            ),
            batch,
        )
        // send proof to Verifier contract
    }

    pub fn create_batch(&mut self) -> Batch<F> {
        let (transactions, roots): (Vec<Transaction>, Vec<(F, F)>) =
            self.transactions.iter().take(BATCH_SIZE).cloned().unzip();
        let batch = Batch {
            transactions,
            roots,
        };
        self.transactions.drain(..BATCH_SIZE);
        batch
    }

    pub fn create_proof(
        &self,
        batch_tree: SparseMerkleTree<F, H, BATCH_SIZE>,
        t_merkle_proofs: Vec<MerkleProof<F, H, BATCH_SIZE>>,
        sender_receiver_in_state_merkle_proofs: Vec<MerkleProof<F, H, N>>,
        state_roots: Vec<(F, F)>,
    ) -> Proof<F, H, N, BATCH_SIZE> {
        Proof {
            batch_tree,
            t_merkle_proofs,
            sender_receiver_in_state_merkle_proofs,
            state_roots,
        }
    }

    pub fn state_root(&self) -> F {
        self.state_merkle.root()
    }

    pub(crate) fn process_deposits(&mut self, txs: Vec<Transaction>) {
        for t in txs {
            let user = UserData::new(self.index_counter, t.1.amount, t.1.sender_address);
            self.db.insert(user.address, user);
            self.index_counter += 1;

            self.state_merkle
                .update(user.index, user.to_field_element(), &self.hasher)
                .expect("Failed to update user info");

            // Need to add deposits to the transactions vec as well
            // skipped just for easier test implementation
            // self.transactions.push((t, self.state_root()));
        }
    }

    #[cfg(test)]
    pub(crate) fn add_withdrawal_address(&mut self, address: PublicKey) {
        let user = UserData::new(self.index_counter, 0, address);
        self.db.insert(user.address, user);
        self.index_counter += 1;

        self.state_merkle
            .update(user.index, user.to_field_element(), &self.hasher)
            .expect("Failed to withdrawal address");
    }
}
