use crate::plonk::proof::Evaluations;
use crate::plonk::utils::delta;

use poly_commit::{Coefficients, Commitment, PointsValue};
use zkstd::common::{vec, CurveAffine, FftField, PrimeField, Vec};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VerificationKey<A: CurveAffine> {
    pub q_range: Commitment<A>,
}
impl<A: CurveAffine> VerificationKey<A> {
    pub fn linearize(
        &self,
        range_separation_challenge: &A::Scalar,
        evaluations: &Evaluations<A::Scalar>,
    ) -> (Vec<A::Scalar>, Vec<A>) {
        let four = A::Scalar::from(4);

        let kappa = range_separation_challenge.square();
        let kappa_sq = kappa.square();
        let kappa_cu = kappa_sq * kappa;

        let b_1 = delta(evaluations.c_eval - (four * evaluations.d_eval));
        let b_2 = delta(evaluations.b_eval - four * evaluations.c_eval) * kappa;
        let b_3 = delta(evaluations.a_eval - four * evaluations.b_eval) * kappa_sq;
        let b_4 = delta(evaluations.d_next_eval - (four * evaluations.a_eval)) * kappa_cu;

        (
            vec![(b_1 + b_2 + b_3 + b_4) * range_separation_challenge],
            vec![self.q_range.0],
        )
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ProvingKey<F: FftField> {
    pub q_range: (Coefficients<F>, PointsValue<F>),
}

impl<F: FftField> ProvingKey<F> {
    #[allow(clippy::too_many_arguments)]
    pub fn compute_quotient_i(
        &self,
        index: usize,
        range_separation_challenge: &F,
        a_w_i: &F,
        b_w_i: &F,
        c_w_i: &F,
        d_w_i: &F,
        d_w_i_next: &F,
    ) -> F {
        let four = F::from(4);
        let q_range_i = &self.q_range.1[index];

        let kappa = range_separation_challenge.square();
        let kappa_sq = kappa.square();
        let kappa_cu = kappa_sq * kappa;

        // Delta([c(X) - 4 * d(X)]) + Delta([b(X) - 4 * c(X)]) + Delta([a(X) - 4
        // * b(X)]) + Delta([d(Xg) - 4 * a(X)]) * Q_Range(X)
        //
        let b_1 = delta(*c_w_i - four * d_w_i);
        let b_2 = delta(*b_w_i - four * c_w_i) * kappa;
        let b_3 = delta(*a_w_i - four * b_w_i) * kappa_sq;
        let b_4 = delta(*d_w_i_next - four * a_w_i) * kappa_cu;
        (b_1 + b_2 + b_3 + b_4) * q_range_i * range_separation_challenge
    }

    pub fn linearize(
        &self,
        range_separation_challenge: &F,
        a_eval: &F,
        b_eval: &F,
        c_eval: &F,
        d_eval: &F,
        d_next_eval: &F,
    ) -> Coefficients<F> {
        let four = F::from(4);
        let q_range_poly = &self.q_range.0;

        let kappa = range_separation_challenge.square();
        let kappa_sq = kappa.square();
        let kappa_cu = kappa_sq * kappa;

        // Delta([c_eval - 4 * d_eval]) + Delta([b_eval - 4 * c_eval]) +
        // Delta([a_eval - 4 * b_eval]) + Delta([d_next_eval - 4 * a_eval]) *
        // Q_Range(X)
        let b_1 = delta(*c_eval - four * d_eval);
        let b_2 = delta(*b_eval - four * c_eval) * kappa;
        let b_3 = delta(*a_eval - four * b_eval) * kappa_sq;
        let b_4 = delta(*d_next_eval - four * a_eval) * kappa_cu;

        let t = (b_1 + b_2 + b_3 + b_4) * range_separation_challenge;

        q_range_poly * &t
    }
}
