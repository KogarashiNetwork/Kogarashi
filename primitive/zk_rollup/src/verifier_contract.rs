use std::marker::PhantomData;

use zkstd::common::FftField;

use crate::{poseidon::FieldHasher, proof::Proof};

#[derive(Default)]
pub(crate) struct VerifierContract<
    F: FftField,
    H: FieldHasher<F, 2>,
    const N: usize,
    const BATCH_SIZE: usize,
> {
    marker: PhantomData<F>,
    marker1: PhantomData<H>,
}

impl<F: FftField, H: FieldHasher<F, 2>, const N: usize, const BATCH_SIZE: usize>
    VerifierContract<F, H, N, BATCH_SIZE>
{
    pub fn verify_proof(
        &self,
        proof: Proof<F, H, N, BATCH_SIZE>,
        // pre_state_root: F,
        // post_state_root: F,
        // batch_root: F,
        // transactions: Vec<Transaction>,
    ) -> bool {
        true
    }
}
