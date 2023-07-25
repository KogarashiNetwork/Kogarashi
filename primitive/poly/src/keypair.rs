use crate::commitment::Commitment;
use crate::poly::Polynomial;
use crate::util;
use crate::witness::Witness;
use ec_pairing::msm_variable_base;
use parity_scale_codec::{Decode, Encode};
use zkstd::behave::*;
use zkstd::common::*;

// key pair structure
#[derive(Clone, Debug, PartialEq, Decode, Encode)]
#[allow(dead_code)]
pub struct KeyPair<P: Pairing> {
    pub(crate) g1: Vec<P::G1Affine>,
    pub(crate) g2: P::G2Affine,
    pub(crate) beta_h: P::G2Affine,
}

impl<P: Pairing> KeyPair<P> {
    const ADDED_BLINDING_DEGREE: usize = 6;

    // setup polynomial evaluation domain
    pub fn setup(k: u64, r: P::ScalarField) -> Self {
        // G1, r * G1, r^2 * G1, ..., r^n-1 * G1
        let g1 = (0..=((1 << k) + Self::ADDED_BLINDING_DEGREE as u64))
            .map(|i| {
                let tw = P::G1Projective::ADDITIVE_GENERATOR * r.pow(i);
                P::G1Affine::from(tw)
            })
            .collect::<Vec<_>>();
        let g2 = P::G2Affine::from(P::G2Projective::ADDITIVE_GENERATOR * r);

        Self {
            g1,
            g2,
            beta_h: P::G2Affine::from(P::G2Projective::from(g2) * r),
        }
    }

    // commit polynomial to g1 projective group
    pub fn commit(&self, poly: &Polynomial<P::ScalarField>) -> Result<Commitment<P>, Error> {
        self.check_commit_degree_is_within_bounds(poly.degree())?;

        Ok(Commitment::new(msm_variable_base::<P>(&self.g1, poly)))
    }

    fn check_commit_degree_is_within_bounds(&self, poly_degree: usize) -> Result<(), Error> {
        match (poly_degree == 0, poly_degree > self.max_degree()) {
            (true, _) => Err(Error::PolynomialDegreeIsZero),
            (false, true) => Err(Error::PolynomialDegreeTooLarge),
            (false, false) => Ok(()),
        }
    }

    pub fn commit_key(&self) -> &Vec<P::G1Affine> {
        &self.g1
    }

    pub fn opening_key(&self) -> P::G2Affine {
        self.g2
    }

    pub fn beta_h(&self) -> P::G2Affine {
        self.beta_h
    }

    pub fn max_degree(&self) -> usize {
        self.g1.len() - 1
    }

    pub fn trim(&self, mut truncated_degree: usize) -> Self {
        truncated_degree += Self::ADDED_BLINDING_DEGREE;
        assert_ne!(truncated_degree, 0);
        assert!(truncated_degree <= self.max_degree());
        if truncated_degree == 1 {
            truncated_degree += 1
        };

        let g1_trunc = self.g1[..=truncated_degree].to_vec();

        Self {
            g1: g1_trunc,
            g2: self.g2,
            beta_h: self.beta_h,
        }
    }

    // create witness for f(a)
    pub fn create_witness(
        &self,
        poly: &Polynomial<P::ScalarField>,
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
            P::G2Projective::from(self.g2) - P::G2Projective::ADDITIVE_GENERATOR * at,
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
        polynomials: &[Polynomial<P::ScalarField>],
        point: &P::ScalarField,
        v_challenge: &P::ScalarField,
    ) -> Polynomial<P::ScalarField> {
        let powers = util::powers_of::<P>(v_challenge, polynomials.len() - 1);

        assert_eq!(powers.len(), polynomials.len());

        let numerator: Polynomial<P::ScalarField> = polynomials
            .iter()
            .zip(powers.iter())
            .map(|(poly, v_challenge)| poly * v_challenge)
            .sum();

        numerator.divide(point)
    }
}

#[derive(Debug)]
pub enum Error {
    PolynomialDegreeIsZero,
    PolynomialDegreeTooLarge,
}
