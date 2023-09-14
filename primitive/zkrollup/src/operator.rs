use crate::{
    db::Db,
    domain::{RollupTransactionInfo, Transaction, UserData},
    get_rng,
    merkle_tree::{MerkleProof, SparseMerkleTree},
    poseidon::FieldHasher,
    BatchCircuit,
};
use ark_std::rand::Rng;
use poly_commit::KzgParams;
use red_jubjub::PublicKey;
use zero_plonk::{prelude::Compiler, proof_system::Proof};
use zkstd::common::{vec, Decode, Encode, FftField, Pairing, SigUtils, Vec};

pub trait BatchGetter<F: FftField> {
    fn final_root(&self) -> F;
}

#[derive(Debug, PartialEq, Eq, Clone, Encode, Decode)]
pub struct Batch<
    P: Pairing,
    H: FieldHasher<P::ScalarField, 2>,
    const N: usize,
    const BATCH_SIZE: usize,
> {
    pub(crate) transactions: Vec<RollupTransactionInfo<P, H, N>>,
}

impl<P: Pairing, H: FieldHasher<P::ScalarField, 2>, const N: usize, const BATCH_SIZE: usize>
    BatchGetter<P::ScalarField> for Batch<P, H, N, BATCH_SIZE>
{
    fn final_root(&self) -> P::ScalarField {
        self.transactions
            .iter()
            .last()
            .map(|data| data.post_root)
            .unwrap()
    }
}

impl<P: Pairing, H: FieldHasher<P::ScalarField, 2>, const N: usize, const BATCH_SIZE: usize> Default
    for Batch<P, H, N, BATCH_SIZE>
{
    fn default() -> Self {
        Self {
            transactions: vec![RollupTransactionInfo::default(); BATCH_SIZE],
        }
    }
}

impl<P: Pairing, H: FieldHasher<P::ScalarField, 2>, const N: usize, const BATCH_SIZE: usize>
    Batch<P, H, N, BATCH_SIZE>
{
    pub fn raw_transactions(&self) -> impl Iterator<Item = &Transaction<P>> {
        self.transactions.iter().map(|info| &info.transaction)
    }

    pub fn intermediate_roots(&self) -> Vec<(P::ScalarField, P::ScalarField)> {
        self.transactions
            .iter()
            .map(|data| (data.pre_root, data.post_root))
            .collect()
    }

    pub fn border_roots(&self) -> (P::ScalarField, P::ScalarField) {
        (self.first_root(), self.final_root())
    }

    pub(crate) fn first_root(&self) -> P::ScalarField {
        self.transactions
            .iter()
            .last()
            .map(|data| data.pre_root)
            .unwrap()
    }
}

#[derive(Default)]
pub struct RollupOperator<
    P: Pairing,
    H: FieldHasher<P::ScalarField, 2>,
    const N: usize,
    const BATCH_SIZE: usize,
> {
    state_merkle: SparseMerkleTree<P::ScalarField, H, N>,
    db: Db<P>,
    transactions: Vec<RollupTransactionInfo<P, H, N>>,
    index_counter: u64,
    hasher: H,
    pp: KzgParams<P>,
}

impl<P: Pairing, H: FieldHasher<P::ScalarField, 2>, const N: usize, const BATCH_SIZE: usize>
    RollupOperator<P, H, N, BATCH_SIZE>
{
    // const BATCH_SIZE: usize = 2;

    pub fn new(hasher: H, pp: KzgParams<P>) -> Self {
        Self {
            state_merkle: SparseMerkleTree::new_empty(&hasher, &[0; 64])
                .expect("Failed to create state merkle tree"),
            hasher,
            pp,
            ..Default::default()
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn execute_transaction(
        &mut self,
        transaction: Transaction<P>,
    ) -> Option<((Proof<P>, Vec<P::ScalarField>), Batch<P, H, N, BATCH_SIZE>)> {
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
            post_sender.nonce = get_rng().gen();

            self.state_merkle
                .update(
                    pre_sender.index,
                    post_sender.to_field_element(),
                    &self.hasher,
                )
                .expect("Failed to update balance");

            let post_receiver = self.db.get_mut(&transaction_data.receiver_address);
            post_receiver.balance += transaction_data.amount;
            post_receiver.nonce = get_rng().gen();

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

    #[allow(clippy::type_complexity)]
    pub fn process_batch(
        &mut self,
    ) -> ((Proof<P>, Vec<P::ScalarField>), Batch<P, H, N, BATCH_SIZE>) {
        let batch = self.create_batch();
        let batch_leaves: Vec<P::ScalarField> = batch
            .raw_transactions()
            .map(|t| t.to_field_element())
            .collect();
        let batch_tree = SparseMerkleTree::<P::ScalarField, H, BATCH_SIZE>::new_sequential(
            &batch_leaves,
            &self.hasher,
            &[0; 64],
        )
        .expect("Failed to create batch merkle tree");

        let t_merkle_proofs: Vec<MerkleProof<P::ScalarField, H, BATCH_SIZE>> =
            (0..batch.transactions.len())
                .map(|index| batch_tree.generate_membership_proof(index as u64))
                .collect();
        let state_roots = batch.intermediate_roots();

        (self.create_proof(batch.clone()), batch)
        // send proof to Verifier contract
    }

    pub fn create_batch(&mut self) -> Batch<P, H, N, BATCH_SIZE> {
        let batch = Batch {
            transactions: self.transactions[0..BATCH_SIZE]
                .try_into()
                .expect("Failed to get batch transactions from slice"),
        };
        self.transactions.drain(..BATCH_SIZE);
        batch
    }

    pub fn create_proof(
        &mut self,
        batch: Batch<P, H, N, BATCH_SIZE>,
    ) -> (Proof<P>, Vec<P::ScalarField>) {
        let label = b"verify";
        let batch_circuit = BatchCircuit::new(batch);
        let prover = Compiler::compile::<BatchCircuit<P, H, N, BATCH_SIZE>, P>(&mut self.pp, label)
            .expect("failed to compile circuit");
        prover
            .0
            .prove(&mut get_rng(), &batch_circuit)
            .expect("failed to prove")
    }

    pub fn state_root(&self) -> P::ScalarField {
        self.state_merkle.root()
    }

    pub fn process_deposit(&mut self, t: Transaction<P>) {
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

    pub fn add_withdraw_address(&mut self, address: PublicKey<P>) {
        let user = UserData::new(self.index_counter, 0, address);
        self.db.insert(user.address, user);
        self.index_counter += 1;

        self.state_merkle
            .update(user.index, user.to_field_element(), &self.hasher)
            .expect("Failed to withdrawal address");
    }
}
