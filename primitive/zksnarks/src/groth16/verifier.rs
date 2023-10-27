use crate::error::Error;
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
    pub fn verify(&self, proof: &Proof<P>) -> Result<(), Error> {
        proof.verify(&self.opening_key)
    }
}
