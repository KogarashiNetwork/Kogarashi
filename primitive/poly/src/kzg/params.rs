use zkstd::common::Pairing;

use super::{CommitmentKey, EvaluationKey};

pub trait PublicParameters<P: Pairing> {
    const ADDITIONAL_DEGREE: usize;

    fn setup(k: u64, r: P::ScalarField) -> Self;

    fn key_pair(self) -> (EvaluationKey<P>, CommitmentKey<P::G1Affine>);
}
