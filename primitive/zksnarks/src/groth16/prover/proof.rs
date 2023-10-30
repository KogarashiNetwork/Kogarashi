use crate::error::Error;
use crate::groth16::key::{PreparedVerifyingKey, VerifyingKey};
use poly_commit::EvaluationKey;
use zkstd::common::{CurveExtended, Group, Pairing, PairingRange, WeierstrassAffine};

pub struct Proof<P: Pairing> {
    pub(crate) a: P::G1Affine,
    pub(crate) b: P::G2Affine,
    pub(crate) c: P::G1Affine,
}

impl<P: Pairing> Proof<P> {
    pub(crate) fn verify(
        &self,
        vk: PreparedVerifyingKey<P>,
        // opening_key: &EvaluationKey<P>,
        public_inputs: &[P::ScalarField],
    ) -> Result<(), Error> {
        let mut acc = P::G1Projective::from(vk.ic[0]);

        for (&i, &b) in public_inputs.iter().zip(vk.ic.iter().skip(1)) {
            acc += b * i;
        }

        // let pairing = P::multi_miller_loop(&[
        //     (self.a, P::G2PairngRepr::from(self.b)),
        //     (-self.c, opening_key.prepared_h.clone()),
        // ])
        // .final_exp();

        let pairing = P::multi_miller_loop(&[
            (self.a, P::G2PairngRepr::from(self.b)),
            (acc.to_affine(), vk.neg_gamma_g2),
            (self.c, vk.neg_delta_g2),
        ])
        .final_exp();

        if pairing != vk.alpha_g1_beta_g2 {
            return Err(Error::ProofVerificationError);
        }
        Ok(())
    }
}
