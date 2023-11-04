use crate::error::Error;
use crate::groth16::key::PreparedVerifyingKey;
use crate::groth16::prover::Proof;
use zkstd::common::Pairing;

// Verify proofs of a given circuit
pub struct Verifier<P: Pairing> {
    pub(crate) vk: PreparedVerifyingKey<P>,
}

impl<P: Pairing> Verifier<P> {
    /// Verify a generated proof
    pub fn verify(&self, proof: &Proof<P>, public_inputs: &[P::ScalarField]) -> Result<(), Error> {
        proof.verify(&self.vk, public_inputs)
    }
}
