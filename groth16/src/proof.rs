use crate::error::Error;
use crate::verifier::PreparedVerifyingKey;

use bls_12_381::{Fr, G1Affine, G1Projective, G2Affine, G2PairingAffine, TatePairing};
use zkstd::common::BNProjective;

pub struct Proof {
    pub(crate) a: G1Affine,
    pub(crate) b: G2Affine,
    pub(crate) c: G1Affine,
}

impl Proof {
    pub(crate) fn verify(
        &self,
        vk: &PreparedVerifyingKey,
        public_inputs: &[Fr],
    ) -> Result<(), Error> {
        if (public_inputs.len() + 1) != vk.ic.len() {
            return Err(Error::InconsistentPublicInputsLen {
                expected: vk.ic.len() - 1,
                provided: public_inputs.len(),
            });
        }
        let mut acc = G1Projective::from(vk.ic[0]);

        for (&i, &b) in public_inputs.iter().zip(vk.ic.iter().skip(1)) {
            acc += b * i;
        }

        // The original verification equation is:
        // A * B = alpha * beta + inputs * gamma + C * delta
        // ... however, we rearrange it so that it is:
        // A * B - inputs * gamma - C * delta = alpha * beta
        // or equivalently:
        // A * B + inputs * (-gamma) + C * (-delta) = alpha * beta
        // which allows us to do a single final exponentiation.

        let pairing = TatePairing::multi_miller_loop(&[
            (self.a, G2PairingAffine::from(self.b)),
            (acc.to_affine(), vk.neg_gamma_g2.clone()),
            (self.c, vk.neg_delta_g2.clone()),
        ])
        .final_exp();

        if pairing != vk.alpha_g1_beta_g2 {
            return Err(Error::ProofVerificationError);
        }
        Ok(())
    }
}
