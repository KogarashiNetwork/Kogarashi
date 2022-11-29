use crate::poly::Polynomial;
use zero_crypto::behave::*;

// key pair structure
#[derive(Clone, Debug)]
pub struct KeyPair<C: Commitment> {
    k: u64,
    pub(crate) g1: Vec<C::G1Affine>,
    pub(crate) g2: Vec<C::G2Affine>,
}

impl<C: Commitment> KeyPair<C> {
    // setup polynomial evaluation domain
    pub fn setup(k: u64, r: C::ScalarField) -> Self {
        // G1, r * G1, r^2 * G1, ..., r^n-1 * G1
        let g1 = (0..(1 << k))
            .map(|i| {
                let tw = C::G1Projective::GENERATOR * r.pow(i);
                C::G1Affine::from(tw)
            })
            .collect::<Vec<_>>();

        let g2 = (0..(1 << k))
            .map(|i| {
                let tw = C::G2Projective::GENERATOR * r.pow(i);
                C::G2Affine::from(tw)
            })
            .collect::<Vec<_>>();

        Self { k, g1, g2 }
    }

    // commit polynomial to g1 projective group
    pub fn commit_to_g1(&self, poly: &Polynomial<C::ScalarField>) -> C::G1Projective {
        assert!(poly.0.len() == self.g1.len());

        poly.0
            .iter()
            .zip(self.g1.iter().rev())
            .fold(C::G1Projective::IDENTITY, |acc, (coeff, base)| {
                acc + C::G1Projective::from(*base) * *coeff
            })
    }

    // commit polynomial to g2 projective group
    pub fn commit_to_g2(&self, poly: &Polynomial<C::ScalarField>) -> C::G2Projective {
        assert!(poly.0.len() == self.g2.len());

        poly.0
            .iter()
            .zip(self.g2.iter().rev())
            .fold(C::G2Projective::IDENTITY, |acc, (coeff, base)| {
                acc + C::G2Projective::from(*base) * *coeff
            })
    }
}
