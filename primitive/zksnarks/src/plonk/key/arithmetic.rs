use crate::plonk::proof::Evaluations;

use poly_commit::Commitment;
use zkstd::common::Affine;

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct VerifierKey<A: Affine> {
    pub q_m: Commitment<A>,
    pub q_l: Commitment<A>,
    pub q_r: Commitment<A>,
    pub q_o: Commitment<A>,
    pub q_4: Commitment<A>,
    pub q_c: Commitment<A>,
    pub q_arith: Commitment<A>,
}

impl<A: Affine> VerifierKey<A> {
    pub fn linearize(&self, evaluations: Evaluations<A::Scalar>) {}
}
