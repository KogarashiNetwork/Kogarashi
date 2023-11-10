use r1cs::{CircuitDriver, DenseVectors};
use zkstd::common::{BNAffine, Group, RngCore};

pub struct PedersenCommitment<C: BNAffine> {
    h: C,
    g: Vec<C>,
}

impl<C: BNAffine> PedersenCommitment<C> {
    pub(crate) fn new(n: u64, mut r: impl RngCore) -> Self {
        let h = C::random(&mut r).into();
        let g = (0..=1 << n).map(|_| C::random(&mut r).into()).collect();
        Self { h, g }
    }

    pub(crate) fn commit(&self, m: &DenseVectors<C::Scalar>, r: &C::Scalar) -> C {
        (self.h * r
            + m.iter()
                .zip(self.g.iter())
                .fold(C::Extended::ADDITIVE_IDENTITY, |sum, (v, e)| sum + *e * v))
        .into()
    }
}
