use zkstd::common::{BNAffine, Group, RngCore};
use zkstd::matrix::DenseVectors;

#[derive(Clone)]
pub struct PedersenCommitment<C: BNAffine> {
    g: Vec<C>,
}

impl<C: BNAffine> PedersenCommitment<C> {
    pub fn new(n: u64, mut r: impl RngCore) -> Self {
        let g = (0..=1 << n).map(|_| C::random(&mut r)).collect();
        Self { g }
    }

    pub(crate) fn commit(&self, m: &DenseVectors<C::Scalar>) -> C {
        (m.iter()
            .zip(self.g.iter())
            .fold(C::Extended::ADDITIVE_IDENTITY, |sum, (v, e)| sum + *e * v))
        .into()
    }
}
