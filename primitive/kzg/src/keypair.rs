use crate::commitment::Commitment;
use crate::poly::Polynomial;
use crate::util;
use crate::witness::Witness;
use zero_crypto::behave::*;
use zero_crypto::common::Vec;

// key pair structure
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct KeyPair<P: Pairing> {
    k: u64,
    pub(crate) g1: Vec<P::G1Affine>,
    pub(crate) g2: P::G2Affine,
}

impl<P: Pairing> KeyPair<P> {
    // setup polynomial evaluation domain
    pub fn setup(k: u64, r: P::ScalarField) -> Self {
        // G1, r * G1, r^2 * G1, ..., r^n-1 * G1
        let g1 = (0..(1 << k))
            .map(|i| {
                let tw = P::G1Projective::ADDITIVE_GENERATOR * r.pow(i);
                P::G1Affine::from(tw)
            })
            .collect::<Vec<_>>();
        let g2 = P::G2Affine::from(P::G2Projective::ADDITIVE_GENERATOR * r);

        Self { k, g1, g2 }
    }

    // commit polynomial to g1 projective group
    pub fn commit(&self, poly: &Polynomial<P::ScalarField>) -> Commitment<P> {
        assert!(poly.0.len() <= self.g1.len());
        let diff = self.g1.len() - poly.0.len();

        Commitment(P::G1Affine::from(
            poly.0
                .iter()
                .rev()
                .zip(self.g1.iter().rev().skip(diff))
                .fold(P::G1Projective::ADDITIVE_IDENTITY, |acc, (coeff, base)| {
                    acc + P::G1Projective::from(*base) * *coeff
                }),
        ))
    }

    pub fn commit_key(&self) -> &Vec<P::G1Affine> {
        &self.g1
    }

    pub fn opening_key(&self) -> P::G2Affine {
        self.g2
    }

    pub fn max_degree(&self) -> u64 {
        self.k
    }

    pub fn trim(&mut self, mut truncated_degree: usize) {
        assert_ne!(truncated_degree, 0);
        assert!((truncated_degree as u64) < self.k);
        if truncated_degree == 1 {
            truncated_degree += 1
        };
        self.g1.resize(truncated_degree, P::G1Affine::default());
    }

    // create witness for f(a)
    pub fn create_witness(
        self,
        poly: &Polynomial<P::ScalarField>,
        at: P::ScalarField,
    ) -> Witness<P> {
        // p(x) - p(at) / x - at
        let quotient = poly.divide(&at);

        // p(s)
        let s_eval = self.commit(poly);
        // p(at)
        let a_eval = P::G1Projective::ADDITIVE_GENERATOR * poly.evaluate(&at);
        // p(s) - p(at) / s - at
        let q_eval = self.commit(&quotient);
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
        let powers = util::powers_of::<P>(&v_challenge, polynomials.len() - 1);

        assert_eq!(powers.len(), polynomials.len());

        let numerator: Polynomial<P::ScalarField> = polynomials
            .iter()
            .zip(powers.iter())
            .map(|(poly, v_challenge)| poly.clone() * *v_challenge)
            .sum();

        numerator.divide(point)
    }
}
