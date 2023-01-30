use crate::poly::Polynomial;
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
    pub fn commit(&self, poly: &Polynomial<P::ScalarField>) -> P::G1Projective {
        assert!(poly.0.len() <= self.g1.len());
        let diff = self.g1.len() - poly.0.len();

        poly.0
            .iter()
            .rev()
            .zip(self.g1.iter().rev().skip(diff))
            .fold(P::G1Projective::ADDITIVE_IDENTITY, |acc, (coeff, base)| {
                acc + P::G1Projective::from(*base) * *coeff
            })
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
            c_eval: P::G1Affine::from(s_eval - a_eval),
            q_eval: P::G1Affine::from(q_eval),
            denominator: P::G2PairngRepr::from(-denominator),
            h: P::G2PairngRepr::from(P::G2Affine::from(P::G2Projective::ADDITIVE_GENERATOR)),
        }
    }
}
