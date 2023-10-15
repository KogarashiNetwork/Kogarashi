use super::error::PlonkError;
use crate::public_params::PublicParameters;

use poly_commit::{powers_of, Coefficients, Commitment, CommitmentKey, EvaluationKey, Witness};
use zkstd::common::*;

/// Kate polynomial commitment params used for prover polynomial domain and proof verification
#[derive(Clone, Debug, PartialEq, Decode, Encode, Default)]
#[allow(dead_code)]
pub struct PlonkParams<P: Pairing> {
    pub(crate) commitment_key: CommitmentKey<P::G1Affine>,
    pub(crate) evaluation_key: EvaluationKey<P>,
}

impl<P: Pairing> PublicParameters<P> for PlonkParams<P> {
    const ADDITIONAL_DEGREE: usize = 6;

    /// setup polynomial evaluation domain
    fn setup(k: u64, r: impl RngCore) -> Self {
        let r = P::ScalarField::random(r);
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

impl<P: Pairing> PlonkParams<P> {
    /// commit polynomial to g1 projective group
    pub fn commit(
        &self,
        poly: &Coefficients<P::ScalarField>,
    ) -> Result<Commitment<P::G1Affine>, PlonkError> {
        if poly.degree() > self.max_degree() {
            Err(PlonkError::CoefficientsDegreeTooLarge)
        } else {
            Ok(self.commitment_key.commit(poly))
        }
    }

    pub fn verification_key(&self) -> EvaluationKey<P> {
        self.evaluation_key.clone()
    }

    pub fn max_degree(&self) -> usize {
        self.commitment_key.bases.len() - 1
    }

    pub fn trim(&self, mut truncated_degree: usize) -> Self {
        truncated_degree += Self::ADDITIONAL_DEGREE;
        assert_ne!(truncated_degree, 0);
        assert!(truncated_degree <= self.max_degree());
        if truncated_degree == 1 {
            truncated_degree += 1
        };

        Self {
            commitment_key: self.commitment_key.trim(truncated_degree),
            evaluation_key: self.evaluation_key.clone(),
        }
    }

    /// create witness for f(a)
    pub fn create_witness(
        &self,
        poly: &Coefficients<P::ScalarField>,
        at: P::ScalarField,
    ) -> Witness<P> {
        // p(x) - p(at) / x - at
        let quotient = poly.divide(&at);

        // p(s)
        let s_eval = self.commit(poly).unwrap();
        // p(at)
        let a_eval = P::G1Projective::ADDITIVE_GENERATOR * poly.evaluate(&at);
        // p(s) - p(at) / s - at
        let q_eval = self.commit(&quotient).unwrap();
        // s - at
        let denominator = P::G2Affine::from(
            P::G2Projective::from(self.evaluation_key.h) - P::G2Projective::ADDITIVE_GENERATOR * at,
        );

        Witness {
            c_eval: P::G1Affine::from(P::G1Projective::from(s_eval.0) - a_eval),
            q_eval: q_eval.0,
            denominator: P::G2PairngRepr::from(-denominator),
            h: P::G2PairngRepr::from(P::G2Affine::from(P::G2Projective::ADDITIVE_GENERATOR)),
        }
    }

    /// Computes a single witness for multiple polynomials at the same point,
    /// by taking a random linear combination of the individual
    /// witnesses. We apply the same optimization mentioned in when
    /// computing each witness; removing f(z).
    pub fn compute_aggregate_witness(
        &self,
        polynomials: &[Coefficients<P::ScalarField>],
        point: &P::ScalarField,
        v_challenge: &P::ScalarField,
    ) -> Coefficients<P::ScalarField> {
        let powers = powers_of(v_challenge, polynomials.len() - 1);

        assert_eq!(powers.len(), polynomials.len());

        let numerator: Coefficients<P::ScalarField> = polynomials
            .iter()
            .zip(powers.iter())
            .map(|(poly, v_challenge)| poly * v_challenge)
            .sum();

        numerator.divide(point)
    }
}
