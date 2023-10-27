use crate::error::Error;
use poly_commit::EvaluationKey;
use zkstd::common::{Group, Pairing, PairingRange};

pub struct Proof<P: Pairing> {
    pub(crate) a: P::G1Affine,
    pub(crate) b: P::G2Affine,
    pub(crate) c: P::G1Affine,
}

impl<P: Pairing> Proof<P> {
    pub(crate) fn verify(&self, opening_key: &EvaluationKey<P>) -> Result<(), Error> {
        let pairing = P::multi_miller_loop(&[
            (self.a, P::G2PairngRepr::from(self.b)),
            (-self.c, opening_key.prepared_h.clone()),
        ])
        .final_exp();

        if pairing != <<P as Pairing>::PairingRange as PairingRange>::Gt::ADDITIVE_IDENTITY {
            return Err(Error::ProofVerificationError);
        }
        Ok(())
    }
}
