use rand_core::RngCore;
use zero_crypto::{behave::*, common::Affine};

// key pair structure
#[derive(Clone, Debug)]
pub struct KeyPair<C: Commitment> {
    k: u64,
    g1: Vec<C::G1Affine>,
}

impl<C: Commitment> KeyPair<C> {
    pub fn new<R: RngCore>(k: u64, rng: R) -> Self {
        let n = 1 << k;
        let g = C::G1Projective::GENERATOR;
        let r = C::ScalarField::random(rng);

        let mut acc = C::ScalarField::one();
        let g1 = (0..n).map(|i| acc * g).collect::<Vec<_>>();

        Self { k, g1 }
    }
}
