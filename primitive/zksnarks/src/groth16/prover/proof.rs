use crate::error::Error;
use crate::groth16::key::PreparedVerifyingKey;
use zkstd::common::{CurveExtended, Pairing, PairingRange};

pub struct Proof<P: Pairing> {
    pub(crate) a: P::G1Affine,
    pub(crate) b: P::G2Affine,
    pub(crate) c: P::G1Affine,
}

impl<P: Pairing> Proof<P> {
    pub(crate) fn verify(
        &self,
        vk: &PreparedVerifyingKey<P>,
        public_inputs: &[P::ScalarField],
    ) -> Result<(), Error> {
        if (public_inputs.len() + 1) != vk.ic.len() {
            return Err(Error::InconsistentPublicInputsLen {
                expected: vk.ic.len() - 1,
                provided: public_inputs.len(),
            });
        }
        let mut acc = P::G1Projective::from(vk.ic[0]);

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

        let pairing = P::multi_miller_loop(&[
            (self.a, P::G2PairngRepr::from(self.b)),
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
