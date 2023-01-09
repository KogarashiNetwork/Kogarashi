use crate::poly::Polynomial;
use crate::witness::Witness;
use zero_crypto::behave::*;

// key pair structure
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct KeyPair<C: Commitment> {
    k: u64,
    pub(crate) g1: Vec<C::G1Affine>,
    pub(crate) g2: C::G2Affine,
}

impl<C: Commitment> KeyPair<C> {
    // setup polynomial evaluation domain
    pub fn setup(k: u64, r: C::ScalarField) -> Self {
        // G1, r * G1, r^2 * G1, ..., r^n-1 * G1
        let g1 = (0..(1 << k))
            .map(|i| {
                let tw = C::G1Projective::ADDITIVE_GENERATOR * r.pow(i);
                C::G1Affine::from(tw)
            })
            .collect::<Vec<_>>();
        let g2 = C::G2Affine::from(C::G2Projective::ADDITIVE_GENERATOR * r);

        Self { k, g1, g2 }
    }

    // commit polynomial to g1 projective group
    pub fn commit(&self, poly: &Polynomial<C::ScalarField>) -> C::G1Projective {
        assert!(poly.0.len() <= self.g1.len());
        let diff = self.g1.len() - poly.0.len();

        poly.0
            .iter()
            .zip(self.g1.iter().rev().skip(diff))
            .fold(C::G1Projective::ADDITIVE_IDENTITY, |acc, (coeff, base)| {
                acc + C::G1Projective::from(*base) * *coeff
            })
    }

    // create witness for f(a)
    pub fn create_witness(
        self,
        poly: &Polynomial<C::ScalarField>,
        at: C::ScalarField,
    ) -> Witness<C> {
        // p(x) - p(at) / x - at
        let quotient = poly.divide(at);
        // p(s)
        let s_eval = self.commit(poly);
        // p(at)
        let a_eval = C::G1Projective::ADDITIVE_GENERATOR * poly.evaluate(at);
        // p(s) - p(at) / s - at
        let q_eval = self.commit(&quotient);
        // s - at
        let denominator = C::G2Projective::from(self.g2) - C::G2Projective::ADDITIVE_GENERATOR * at;

        Witness {
            s_eval: C::G1Affine::from(s_eval),
            a_eval: C::G1Affine::from(a_eval),
            q_eval: C::G1Affine::from(q_eval),
            denominator: C::G2Affine::from(denominator),
        }
    }
}
