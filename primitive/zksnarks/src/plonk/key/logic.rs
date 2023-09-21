use crate::plonk::proof::Evaluations;
use crate::plonk::utils::{delta, delta_xor_and};

use poly_commit::{Coefficients, Commitment, PointsValue};
use zkstd::common::{vec, CurveAffine, FftField, PrimeField, Vec};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VerificationKey<A: CurveAffine> {
    pub q_c: Commitment<A>,
    pub q_logic: Commitment<A>,
}

impl<A: CurveAffine> VerificationKey<A> {
    pub fn linearize(
        &self,
        logic_separation_challenge: &A::Scalar,
        evaluations: &Evaluations<A::Scalar>,
    ) -> (Vec<A::Scalar>, Vec<A>) {
        let four = A::Scalar::from(4);

        let kappa = logic_separation_challenge.square();
        let kappa_sq = kappa.square();
        let kappa_cu = kappa_sq * kappa;
        let kappa_qu = kappa_cu * kappa;

        let a = evaluations.a_next_eval - four * evaluations.a_eval;
        let c_0 = delta(a);

        let b = evaluations.b_next_eval - four * evaluations.b_eval;
        let c_1 = delta(b) * kappa;

        let d = evaluations.d_next_eval - four * evaluations.d_eval;
        let c_2 = delta(d) * kappa_sq;

        let w = evaluations.c_eval;
        let c_3 = (w - a * b) * kappa_cu;

        let c_4 = delta_xor_and(&a, &b, &w, &d, &evaluations.q_c_eval) * kappa_qu;

        (
            vec![(c_0 + c_1 + c_2 + c_3 + c_4) * logic_separation_challenge],
            vec![self.q_logic.0],
        )
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ProvingKey<F: FftField> {
    pub q_c: (Coefficients<F>, PointsValue<F>),
    pub q_logic: (Coefficients<F>, PointsValue<F>),
}

impl<F: FftField> ProvingKey<F> {
    #[allow(clippy::too_many_arguments)]
    pub fn compute_quotient_i(
        &self,
        index: usize,
        logic_separation_challenge: &F,
        a_w_i: &F,
        a_w_i_next: &F,
        b_w_i: &F,
        b_w_i_next: &F,
        c_w_i: &F,
        d_w_i: &F,
        d_w_i_next: &F,
    ) -> F {
        let four = F::from(4);

        let q_logic_i = &self.q_logic.1[index];
        let q_c_i = &self.q_c.1[index];

        let kappa = logic_separation_challenge.square();
        let kappa_sq = kappa.square();
        let kappa_cu = kappa_sq * kappa;
        let kappa_qu = kappa_cu * kappa;

        let a = *a_w_i_next - four * a_w_i;
        let c_0 = delta(a);

        let b = *b_w_i_next - four * b_w_i;
        let c_1 = delta(b) * kappa;

        let d = *d_w_i_next - four * d_w_i;
        let c_2 = delta(d) * kappa_sq;

        let w = c_w_i;
        let c_3 = (*w - a * b) * kappa_cu;

        let c_4 = delta_xor_and(&a, &b, w, &d, q_c_i) * kappa_qu;

        *q_logic_i * (c_3 + c_0 + c_1 + c_2 + c_4) * logic_separation_challenge
    }

    #[allow(clippy::too_many_arguments)]
    pub fn linearize(
        &self,
        logic_separation_challenge: &F,
        a_eval: &F,
        a_next_eval: &F,
        b_eval: &F,
        b_next_eval: &F,
        c_eval: &F,
        d_eval: &F,
        d_next_eval: &F,
        q_c_eval: &F,
    ) -> Coefficients<F> {
        let four = F::from(4);
        let q_logic_poly = &self.q_logic.0;

        let kappa = logic_separation_challenge.square();
        let kappa_sq = kappa.square();
        let kappa_cu = kappa_sq * kappa;
        let kappa_qu = kappa_cu * kappa;

        let a = *a_next_eval - four * a_eval;
        let c_0 = delta(a);

        let b = *b_next_eval - four * b_eval;
        let c_1 = delta(b) * kappa;

        let d = *d_next_eval - four * d_eval;
        let c_2 = delta(d) * kappa_sq;

        let w = c_eval;
        let c_3 = (*w - a * b) * kappa_cu;

        let c_4 = delta_xor_and(&a, &b, w, &d, q_c_eval) * kappa_qu;

        let t = (c_0 + c_1 + c_2 + c_3 + c_4) * logic_separation_challenge;

        q_logic_poly * &t
    }
}
