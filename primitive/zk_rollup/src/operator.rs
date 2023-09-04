use std::vec;

use zkstd::common::{FftField, SigUtils};

use crate::{
    db::Db,
    domain::{RollupTransactionInfo, Transaction, UserData},
    merkle_tree::{MerkleProof, SparseMerkleTree},
    poseidon::FieldHasher,
    proof::Proof,
};
#[cfg(test)]
use red_jubjub::PublicKey;

#[derive(Debug, PartialEq, Default)]
pub(crate) struct Batch<F: FftField, H: FieldHasher<F, 2>, const N: usize> {
    pub(crate) transactions: Vec<RollupTransactionInfo<F, H, N>>,
}

impl<F: FftField, H: FieldHasher<F, 2>, const N: usize> Batch<F, H, N> {
    pub fn raw_transactions(&self) -> impl Iterator<Item = &Transaction> {
        self.transactions.iter().map(|info| &info.transaction)
    }

    pub fn intermediate_roots(&self) -> Vec<(F, F)> {
        self.transactions
            .iter()
            .map(|data| (data.pre_root, data.post_root))
            .collect()
    }

    pub fn border_roots(&self) -> (F, F) {
        (self.first_root(), self.final_root())
    }

    pub(crate) fn first_root(&self) -> F {
        self.transactions
            .iter()
            .last()
            .map(|data| data.pre_root)
            .unwrap()
    }

    pub(crate) fn final_root(&self) -> F {
        self.transactions
            .iter()
            .last()
            .map(|data| data.post_root)
            .unwrap()
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
    transactions: Vec<RollupTransactionInfo<F, H, N>>,
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
    ) -> Option<(Proof<F, H, N, BATCH_SIZE>, Batch<F, H, N>)> {
        let Transaction(signature, transaction_data) = transaction;
        let pre_root = self.state_root();

        let pre_sender = *self.db.get(&transaction_data.sender_address);

        let pre_sender_proof = self
            .state_merkle
            .generate_membership_proof(pre_sender.index);
        assert!(pre_sender_proof
            .check_membership(
                &self.state_merkle.root(),
                &pre_sender.to_field_element(),
                &self.hasher,
            )
            .expect("Sender is not presented in the state"));

        let pre_receiver = *self.db.get(&transaction_data.receiver_address);

        let pre_receiver_proof = self
            .state_merkle
            .generate_membership_proof(pre_receiver.index);
        assert!(pre_receiver_proof
            .check_membership(
                &self.state_merkle.root(),
                &pre_receiver.to_field_element(),
                &self.hasher,
            )
            .expect("Receiver is not presented in the state"));
        assert!(transaction_data
            .sender_address
            .validate(&transaction_data.to_bytes(), signature));

        {
            let post_sender = self.db.get_mut(&transaction_data.sender_address);

            assert!(pre_sender.balance >= transaction_data.amount);
            post_sender.balance -= transaction_data.amount;

            self.state_merkle
                .update(
                    pre_sender.index,
                    post_sender.to_field_element(),
                    &self.hasher,
                )
                .expect("Failed to update balance");

            let post_receiver = self.db.get_mut(&transaction_data.receiver_address);
            post_receiver.balance += transaction_data.amount;

            self.state_merkle
                .update(
                    pre_receiver.index,
                    post_receiver.to_field_element(),
                    &self.hasher,
                )
                .expect("Failed to update balance");
        }

        let post_sender_proof = self
            .state_merkle
            .generate_membership_proof(pre_sender.index);
        let post_receiver_proof = self
            .state_merkle
            .generate_membership_proof(pre_receiver.index);

        self.transactions.push(RollupTransactionInfo {
            transaction,
            pre_root,
            post_root: self.state_root(),
            pre_sender,
            pre_receiver,
            pre_sender_proof,
            pre_receiver_proof,
            post_sender_proof,
            post_receiver_proof,
        });

        if self.transactions.len() >= BATCH_SIZE {
            Some(self.process_batch())
        } else {
            None
        }
    }

    pub fn process_batch(&mut self) -> (Proof<F, H, N, BATCH_SIZE>, Batch<F, H, N>) {
        let batch = self.create_batch();
        let batch_leaves: Vec<F> = batch
            .raw_transactions()
            .map(|t| t.to_field_element())
            .collect();
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

    pub fn create_batch(&mut self) -> Batch<F, H, N> {
        let batch = Batch {
            transactions: (self.transactions[0..BATCH_SIZE]).to_vec(),
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
