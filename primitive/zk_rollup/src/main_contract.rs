use crate::{
    merkle_tree::{MerkleProof, TreeHash},
    operator::Transaction,
};

struct MainContract {
    rollup_state_root: TreeHash,
    deposits: Vec<Transaction>,
}

impl MainContract {
    pub fn deposit(&mut self, amount: u64) {
        let t = Transaction::new(amount);
        self.deposits.push(t);
    }

    pub fn withdraw(
        &self,
        l2_burn_merkle_proof: MerkleProof,
        batch_root: TreeHash,
        transaction: Transaction,
        l1_address: &str,
    ) {
        // merkle_verify(l2_burn_merkle_proof, batch_root);
        // send(transaction.amount, l1_address);
    }

    pub fn update_state(&mut self, new_state_root: TreeHash) {
        self.rollup_state_root = new_state_root;
    }
    pub fn add_batch(&self, compressed_batch_data: Transaction) {
        // calldata <- compressed_batch_data
    }

    pub fn check_balance(&self, merkle_proof: MerkleProof) -> u64 {
        // merkle_verify(merkle_proof, self.rollup_state_root);
        // get_balance()
        0
    }
}
