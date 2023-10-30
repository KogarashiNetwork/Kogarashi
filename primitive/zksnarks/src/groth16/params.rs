use crate::public_params::PublicParameters;
use poly_commit::{CommitmentKey, EvaluationKey};
use zkstd::common::*;

/// Kate polynomial commitment params used for prover polynomial domain and proof verification
#[derive(Clone, Debug, PartialEq, Decode, Encode, Default)]
#[allow(dead_code)]
pub struct Groth16Params<P: Pairing> {
    pub(crate) commitment_key: CommitmentKey<P::G1Affine>,
    pub(crate) evaluation_key: EvaluationKey<P>,
}

impl<P: Pairing> PublicParameters<P> for Groth16Params<P> {
    const ADDITIONAL_DEGREE: usize = 0;

    /// setup polynomial evaluation domain
    fn setup(k: u64, r: impl RngCore) -> Self {
        // let r = P::ScalarField::random(r);
        let r = P::ScalarField::one();
        // G1, r * G1, r^2 * G1, ..., r^n-1 * G1
        let g1 = (0..=((1 << k) + Self::ADDITIONAL_DEGREE as u64))
            .map(|i| {
                let tw = P::G1Projective::ADDITIVE_GENERATOR * r.pow(i);
                P::G1Affine::from(tw)
            })
            .collect::<Vec<_>>();
        let g2 = P::G2Affine::from(P::G2Projective::ADDITIVE_GENERATOR * r);
        let beta_h = P::G2Affine::from(P::G2Projective::from(g2) * r);

        Self {
            commitment_key: CommitmentKey { bases: g1.clone() },
            evaluation_key: EvaluationKey {
                g: g1[0],
                h: g2,
                beta_h,
                prepared_h: P::G2PairngRepr::from(g2),
                prepared_beta_h: P::G2PairngRepr::from(beta_h),
            },
        }
    }
}
