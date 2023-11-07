use crate::error::Error;
use crate::proof::Proof;

use bls_12_381::{Fr, G1Affine, G2Affine, G2PairingAffine, Gt, TatePairing};
use zkstd::common::Vec;

// Verify proofs of a given circuit
pub struct Verifier {
    pub(crate) vk: PreparedVerifyingKey,
}

impl Verifier {
    /// Verify a generated proof
    pub fn verify(&self, proof: &Proof, public_inputs: &[Fr]) -> Result<(), Error> {
        proof.verify(&self.vk, public_inputs)
    }
}

#[derive(Clone, Debug)]
pub struct VerifyingKey {
    // alpha in g1 for verifying and for creating A/C elements of
    // proof. Never the point at infinity.
    pub alpha_g1: G1Affine,

    // beta in g1 and g2 for verifying and for creating B/C elements
    // of proof. Never the point at infinity.
    pub beta_g1: G1Affine,
    pub beta_g2: G2Affine,

    // gamma in g2 for verifying. Never the point at infinity.
    pub gamma_g2: G2Affine,

    // delta in g1/g2 for verifying and proving, essentially the magic
    // trapdoor that forces the prover to evaluate the C element of the
    // proof with only components from the CRS. Never the point at
    // infinity.
    pub delta_g1: G1Affine,
    pub delta_g2: G2Affine,

    // Elements of the form (beta * u_i(tau) + alpha v_i(tau) + w_i(tau)) / gamma
    // for all public inputs. Because all public inputs have a dummy constraint,
    // this is the same size as the number of inputs, and never contains points
    // at infinity.
    pub ic: Vec<G1Affine>,
}

impl VerifyingKey {
    pub(crate) fn prepare(&self) -> PreparedVerifyingKey {
        let gamma = -self.gamma_g2;
        let delta = -self.delta_g2;

        PreparedVerifyingKey {
            alpha_g1_beta_g2: TatePairing::multi_miller_loop(&[(
                self.alpha_g1,
                G2PairingAffine::from(self.beta_g2),
            )])
            .final_exp(),
            neg_gamma_g2: G2PairingAffine::from(gamma),
            neg_delta_g2: G2PairingAffine::from(delta),
            ic: self.ic.clone(),
        }
    }
}

#[derive(Debug)]
pub struct PreparedVerifyingKey {
    /// Pairing result of alpha*beta
    pub(crate) alpha_g1_beta_g2: Gt,
    /// -gamma in G2
    pub(crate) neg_gamma_g2: G2PairingAffine,
    /// -delta in G2
    pub(crate) neg_delta_g2: G2PairingAffine,
    /// Copy of IC from `VerifiyingKey`.
    pub(crate) ic: Vec<G1Affine>,
}
