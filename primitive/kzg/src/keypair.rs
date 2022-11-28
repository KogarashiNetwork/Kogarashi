use crate::poly::Polynomial;
use rand_core::RngCore;
use zero_crypto::behave::*;

// key pair structure
#[derive(Clone, Debug)]
pub struct KeyPair<C: Commitment> {
    k: u64,
    g1: Vec<C::G1Affine>,
    g2: Vec<C::G2Affine>,
}

impl<C: Commitment> KeyPair<C> {
    // setup polynomial evaluation domain
    pub fn setup<R: RngCore>(k: u64, r: C::ScalarField) -> Self {
        let n = 1 << k;
        let g1_g = C::G1Projective::GENERATOR;
        let g2_g = C::G2Projective::GENERATOR;

        let mut acc = C::ScalarField::IDENTITY;
        let g1 = (0..n)
            .map(|_| {
                let res = C::G1Affine::from(g1_g * acc);
                acc *= r;
                res
            })
            .collect::<Vec<_>>();

        let g2 = (0..n)
            .map(|_| {
                let res = C::G2Affine::from(g2_g * acc);
                acc *= r;
                res
            })
            .collect::<Vec<_>>();

        Self { k, g1, g2 }
    }

    // commit polynomial to g1 projective group
    pub fn commit_to_g1(&self, poly: &Polynomial<C::ScalarField>) -> C::G1Projective {
        let mut acc = C::G1Projective::IDENTITY;

        poly.0
            .iter()
            .zip(self.g1.iter())
            .for_each(|(scalar, base)| {
                acc += C::G1Projective::from(*base) * *scalar;
            });
        acc
    }

    // commit polynomial to g2 projective group
    pub fn commit_to_g2(&self, poly: &Polynomial<C::ScalarField>) -> C::G2Projective {
        let mut acc = C::G2Projective::IDENTITY;

        poly.0
            .iter()
            .zip(self.g2.iter())
            .for_each(|(scalar, base)| {
                acc += C::G2Projective::from(*base) * *scalar;
            });
        acc
    }
}
