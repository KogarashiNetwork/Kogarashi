use zkstd::common::{BNAffine, Decode, Encode, Group, RngCore};
use zkstd::matrix::DenseVectors;

#[derive(Clone, PartialEq, Eq, Debug, Encode, Decode)]
pub struct PedersenCommitment<C: BNAffine> {
    g: Vec<C>,
}

impl<C: BNAffine> PedersenCommitment<C> {
    pub fn new<R: RngCore>(n: u64, rng: &mut R) -> Self {
        let g = (0..=1 << n).map(|_| C::random(rng)).collect();
        Self { g }
    }

    pub(crate) fn commit(&self, m: &DenseVectors<C::Scalar>) -> C {
        (m.iter()
            .zip(self.g.iter())
            .fold(C::Extended::ADDITIVE_IDENTITY, |sum, (v, e)| sum + *e * v))
        .into()
    }
}
