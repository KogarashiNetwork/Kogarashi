use crate::{
    merkle_tree::{MerkleProof, MerkleTree, TreeHash},
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

#[derive(Hash)]
pub(crate) struct UserData {
    balance: u64,
    address: String,
}

impl Transaction {
    pub fn new(amount: u64) -> Self {
        Self {
            amount,
            ..Default::default()
        }
    }
}
pub(crate) struct Batch {
    prev_root: TreeHash,
    new_root: TreeHash,
    transactions: Vec<Transaction>,
}

struct RollupOperator {
    state_merkle: MerkleTree<UserData>,
    transactions: Vec<(Transaction, TreeHash)>,
}

impl RollupOperator {
    const BATCH_SIZE: usize = 25;
    pub fn execute_transaction(&mut self, transaction: Transaction) {
        // process transactions
        let new_root = [0; 32];
        self.transactions.push((transaction, new_root));
        if self.transactions.len() > Self::BATCH_SIZE {
            self.create_batch();
            // create merkle tree
            //self.create_proof();
            // send proof to Verifier contract
        }
    }
    pub fn create_batch(&mut self) -> Batch {
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
        batch_tree: MerkleTree<Transaction>,
        t_merkle_proofs: Vec<MerkleProof>,
        sender_receiver_in_state_merkle_proofs: Vec<MerkleProof>,
        state_roots: Vec<TreeHash>,
    ) -> Proof {
        Proof {}
    }
}
