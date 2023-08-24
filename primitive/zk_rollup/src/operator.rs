use zkstd::common::FftField;

use crate::{
    merkle_tree::{MerkleProof, SparseMerkleTree},
    poseidon::FieldHasher,
    proof::Proof,
};

#[derive(Hash, Default, Clone)]
struct Signature;

#[derive(Hash, Default, Clone)]
pub(crate) struct Transaction {
    sender_address: String,
    receiver_address: String,
    signature: Signature,
    amount: u64,
    nonce: u64,
}

#[derive(Hash, Default)]
pub(crate) struct UserData {
    balance: u64,
    address: String,
}

impl UserData {
    pub fn balance(&self) -> u64 {
        self.balance
    }
}

impl Transaction {
    pub fn new(amount: u64) -> Self {
        Self {
            amount,
            ..Default::default()
        }
    }
}
pub(crate) struct Batch<F: FftField> {
    prev_root: F,
    new_root: F,
    transactions: Vec<Transaction>,
}

#[derive(Default)]
pub(crate) struct RollupOperator<F: FftField, H: FieldHasher<F, 2>, const N: usize> {
    state_merkle: SparseMerkleTree<F, H, N>,
    transactions: Vec<(Transaction, F)>,
}

// ACCOUNT_LIMIT -> StateMerkleTree
// BATCH_SIZE -> TransactionMerkleTree

impl<F: FftField, H: FieldHasher<F, 2>, const N: usize> RollupOperator<F, H, N> {
    const BATCH_SIZE: usize = 25;
    pub fn execute_transaction(&mut self, transaction: Transaction) {
        // process transactions
        let new_root = F::zero();
        self.transactions.push((transaction, new_root));
        if self.transactions.len() > Self::BATCH_SIZE {
            self.create_batch();
            // create merkle tree
            //self.create_proof();
            // send proof to Verifier contract
        }
    }
    pub fn create_batch(&mut self) -> Batch<F> {
        let batch = Batch {
            prev_root: self.transactions[0].1,
            new_root: self.transactions[Self::BATCH_SIZE - 1].1,
            transactions: self.transactions[..Self::BATCH_SIZE]
                .iter()
                .map(|(t, _)| t.clone())
                .collect(),
        };
        self.transactions.drain(..Self::BATCH_SIZE);
        batch
    }

    pub fn create_proof(
        &self,
        batch_tree: SparseMerkleTree<F, H, N>,
        t_merkle_proofs: Vec<MerkleProof<F>>,
        sender_receiver_in_state_merkle_proofs: Vec<MerkleProof<F>>,
        state_roots: Vec<F>,
    ) -> Proof {
        Proof {}
    }
}
