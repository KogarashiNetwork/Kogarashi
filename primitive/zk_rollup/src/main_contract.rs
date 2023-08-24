use zkstd::common::FftField;

use crate::{merkle_tree::MerkleProof, operator::Transaction};

#[derive(Default)]
pub(crate) struct MainContract<F: FftField> {
    pub(crate) rollup_state_root: F,
    deposits: Vec<Transaction>,
}

impl<F: FftField> MainContract<F> {
    pub fn deposit(&mut self, amount: u64) {
        let t = Transaction::new(amount);
        self.deposits.push(t);
    }

    pub fn withdraw(
        &self,
        l2_burn_merkle_proof: MerkleProof<F>,
        batch_root: F,
        transaction: Transaction,
        l1_address: &str,
    ) {
        // merkle_verify(l2_burn_merkle_proof, batch_root);
        // send(transaction.amount, l1_address);
    }

    pub fn update_state(&mut self, new_state_root: F) {
        self.rollup_state_root = new_state_root;
    }
    pub fn add_batch(&self, compressed_batch_data: Transaction) {
        // calldata <- compressed_batch_data
    }

    pub fn check_balance(&self, merkle_proof: MerkleProof<F>) -> u64 {
        // merkle_verify(merkle_proof, self.rollup_state_root);
        // get_balance()
        0
    }
}
