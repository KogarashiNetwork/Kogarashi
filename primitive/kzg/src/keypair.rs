use crate::poly::Polynomial;
use rand_core::RngCore;
use zero_crypto::behave::*;

// key pair structure
#[derive(Clone, Debug)]
pub struct KeyPair<C: Commitment> {
    k: u64,
    g1: Vec<C::G1Affine>,
}

impl<C: Commitment> KeyPair<C> {
    pub fn setup<R: RngCore>(k: u64, rng: R) -> Self {
        let n = 1 << k;
        let g = C::G1Projective::GENERATOR;
        let r = C::ScalarField::random(rng);

        let mut acc = C::ScalarField::IDENTITY;
        let g1 = (0..n)
            .map(|_| {
                let res = C::G1Affine::from(g * acc);
                acc *= r;
                res
            })
            .collect::<Vec<_>>();

        Self { k, g1 }
    }

    pub fn commit(&self, poly: &Polynomial<C::ScalarField>) -> C::G1Projective {
        let mut acc = C::G1Projective::IDENTITY;

        poly.0
            .iter()
            .zip(self.g1.iter())
            .for_each(|(scalar, base)| {
                acc += C::G1Projective::from(*base) * *scalar;
            });
        acc
    }
}
