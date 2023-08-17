use crate::{merkle_tree::TreeHash, operator::Transaction, proof::Proof};

struct VerifierContract {}

impl VerifierContract {
    pub fn verify_proof(
        proof: Proof,
        pre_state_root: TreeHash,
        post_state_root: TreeHash,
        batch_root: TreeHash,
        transactions: Vec<Transaction>,
    ) {
    }
}
