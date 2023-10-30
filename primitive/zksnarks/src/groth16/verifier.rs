use crate::error::Error;
use crate::groth16::key::PreparedVerifyingKey;
use crate::groth16::prover::Proof;
use poly_commit::EvaluationKey;
use zkstd::common::Pairing;

// Verify proofs of a given circuit
pub struct Verifier<P: Pairing> {
    pub(crate) opening_key: EvaluationKey<P>,
}

impl<P: Pairing> Verifier<P> {
    pub(crate) fn new(opening_key: EvaluationKey<P>) -> Self {
        Self { opening_key }
    }

    /// Verify a generated proof
    pub fn verify(
        &self,
        vk: PreparedVerifyingKey<P>,
        proof: &Proof<P>,
        public_inputs: &[P::ScalarField],
    ) -> Result<(), Error> {
        proof.verify(vk, public_inputs)
    }
}
