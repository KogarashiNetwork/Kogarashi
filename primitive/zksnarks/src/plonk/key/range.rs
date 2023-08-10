use crate::plonk::proof::Evaluations;

use poly_commit::Commitment;
use zkstd::common::{vec, Affine, PrimeField, Vec};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VerificationKey<A: Affine> {
    pub q_range: Commitment<A>,
}
impl<A: Affine> VerificationKey<A> {
    pub fn linearize(
        &self,
        range_separation_challenge: &A::Scalar,
        evaluations: &Evaluations<A::Scalar>,
    ) -> (Vec<A::Scalar>, Vec<A>) {
        let four = A::Scalar::from(4);

        let kappa = range_separation_challenge.square();
        let kappa_sq = kappa.square();
        let kappa_cu = kappa_sq * kappa;

        let b_1 = delta::<A::Scalar>(evaluations.c_eval - (four * evaluations.d_eval));
        let b_2 = delta::<A::Scalar>(evaluations.b_eval - four * evaluations.c_eval) * kappa;
        let b_3 = delta::<A::Scalar>(evaluations.a_eval - four * evaluations.b_eval) * kappa_sq;
        let b_4 =
            delta::<A::Scalar>(evaluations.d_next_eval - (four * evaluations.a_eval)) * kappa_cu;

        (
            vec![(b_1 + b_2 + b_3 + b_4) * range_separation_challenge],
            vec![self.q_range.0],
        )
    }
}

// Computes f(f-1)(f-2)(f-3)
fn delta<F: PrimeField>(f: F) -> F {
    let f_1 = f - F::one();
    let f_2 = f - F::from(2);
    let f_3 = f - F::from(3);
    f * f_1 * f_2 * f_3
}
