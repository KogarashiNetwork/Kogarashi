use std::marker::PhantomData;

use zkstd::common::FftField;

use crate::{operator::Transaction, proof::Proof};

pub(crate) struct VerifierContract<F: FftField> {
    marker: PhantomData<F>,
}

impl<F: FftField> VerifierContract<F> {
    pub fn verify_proof(
        proof: Proof,
        pre_state_root: F,
        post_state_root: F,
        batch_root: F,
        transactions: Vec<Transaction>,
    ) {
    }
}
