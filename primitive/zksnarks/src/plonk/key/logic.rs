use crate::plonk::proof::Evaluations;

use poly_commit::{Coefficients, Commitment, Evaluations as PolyEval};
use zkstd::common::{vec, Affine, FftField, PrimeField, Vec};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VerificationKey<A: Affine> {
    pub q_c: Commitment<A>,
    pub q_logic: Commitment<A>,
}

impl<A: Affine> VerificationKey<A> {
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
        let c_0 = delta::<A::Scalar>(a);

        let b = evaluations.b_next_eval - four * evaluations.b_eval;
        let c_1 = delta::<A::Scalar>(b) * kappa;

        let d = evaluations.d_next_eval - four * evaluations.d_eval;
        let c_2 = delta::<A::Scalar>(d) * kappa_sq;

        let w = evaluations.c_eval;
        let c_3 = (w - a * b) * kappa_cu;

        let c_4 = delta_xor_and::<A::Scalar>(&a, &b, &w, &d, &evaluations.q_c_eval) * kappa_qu;

        (
            vec![(c_0 + c_1 + c_2 + c_3 + c_4) * logic_separation_challenge],
            vec![self.q_logic.0],
        )
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ProvingKey<F: FftField> {
    pub q_c: (Coefficients<F>, PolyEval<F>),
    pub q_logic: (Coefficients<F>, PolyEval<F>),
}

impl<F: FftField> ProvingKey<F> {
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

// Computes f(f-1)(f-2)(f-3)
fn delta<F: PrimeField>(f: F) -> F {
    let f_1 = f - F::one();
    let f_2 = f - F::from(2);
    let f_3 = f - F::from(3);
    f * f_1 * f_2 * f_3
}

// The identity we want to check is q_logic * A = 0
// A = B + E
// B = q_c * [9c - 3(a+b)]
// E = 3(a+b+c) - 2F
// F = w[w(4w - 18(a+b) + 81) + 18(a^2 + b^2) - 81(a+b) + 83]
#[allow(non_snake_case)]
fn delta_xor_and<F: PrimeField>(a: &F, b: &F, w: &F, c: &F, q_c: &F) -> F {
    let nine = F::from(9);
    let two = F::from(2);
    let three = F::from(3);
    let four = F::from(4);
    let eighteen = F::from(18);
    let eighty_one = F::from(81);
    let eighty_three = F::from(83);

    let F = *w
        * (*w * (four * w - eighteen * (*a + b) + eighty_one)
            + eighteen * (a.square() + b.square())
            - eighty_one * (*a + b)
            + eighty_three);
    let E = three * (*a + b + c) - (two * F);
    let B = *q_c * ((nine * c) - three * (*a + b));
    B + E
}
