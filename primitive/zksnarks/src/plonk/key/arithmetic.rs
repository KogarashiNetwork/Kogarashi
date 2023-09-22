use crate::plonk::proof::Evaluations;

use poly_commit::{Coefficients, Commitment, PointsValue};
use zkstd::common::{vec, CurveAffine, FftField, Vec};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VerificationKey<A: CurveAffine> {
    pub q_m: Commitment<A>,
    pub q_l: Commitment<A>,
    pub q_r: Commitment<A>,
    pub q_o: Commitment<A>,
    pub q_4: Commitment<A>,
    pub q_c: Commitment<A>,
    pub q_arith: Commitment<A>,
}

impl<A: CurveAffine> VerificationKey<A> {
    pub fn linearize(&self, evaluations: &Evaluations<A::Scalar>) -> (Vec<A::Scalar>, Vec<A>) {
        let (q_arith_eval, a_eval, b_eval, c_eval, d_eval) = (
            evaluations.q_arith_eval,
            evaluations.a_eval,
            evaluations.b_eval,
            evaluations.c_eval,
            evaluations.d_eval,
        );
        (
            vec![
                a_eval * b_eval * q_arith_eval,
                a_eval * q_arith_eval,
                b_eval * q_arith_eval,
                c_eval * q_arith_eval,
                d_eval * q_arith_eval,
                q_arith_eval,
            ],
            vec![
                self.q_m.0, self.q_l.0, self.q_r.0, self.q_o.0, self.q_4.0, self.q_c.0,
            ],
        )
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ProvingKey<F: FftField> {
    pub q_m: (Coefficients<F>, PointsValue<F>),
    pub q_l: (Coefficients<F>, PointsValue<F>),
    pub q_r: (Coefficients<F>, PointsValue<F>),
    pub q_o: (Coefficients<F>, PointsValue<F>),
    pub q_c: (Coefficients<F>, PointsValue<F>),
    pub q_4: (Coefficients<F>, PointsValue<F>),
    pub q_arith: (Coefficients<F>, PointsValue<F>),
}

impl<F: FftField> ProvingKey<F> {
    pub fn compute_quotient_i(
        &self,
        index: usize,
        a_w_i: &F,
        b_w_i: &F,
        c_w_i: &F,
        d_w_i: &F,
    ) -> F {
        let q_m_i = &self.q_m.1[index];
        let q_l_i = &self.q_l.1[index];
        let q_r_i = &self.q_r.1[index];
        let q_o_i = &self.q_o.1[index];
        let q_c_i = &self.q_c.1[index];
        let q_4_i = &self.q_4.1[index];
        let q_arith_i = &self.q_arith.1[index];

        // (a(x)b(x)q_M(x) + a(x)q_L(x) + b(X)q_R(x) + c(X)q_O(X) + d(x)q_4(X) +
        // Q_C(X)) * Q_Arith(X)
        //
        let a_1 = *a_w_i * b_w_i * q_m_i;
        let a_2 = *a_w_i * q_l_i;
        let a_3 = *b_w_i * q_r_i;
        let a_4 = *c_w_i * q_o_i;
        let a_5 = *d_w_i * q_4_i;
        let a_6 = q_c_i;
        (a_1 + a_2 + a_3 + a_4 + a_5 + a_6) * q_arith_i
    }

    pub fn linearize(
        &self,
        a_eval: &F,
        b_eval: &F,
        c_eval: &F,
        d_eval: &F,
        q_arith_eval: &F,
    ) -> Coefficients<F> {
        let q_m_poly = &self.q_m.0;
        let q_l_poly = &self.q_l.0;
        let q_r_poly = &self.q_r.0;
        let q_o_poly = &self.q_o.0;
        let q_c_poly = &self.q_c.0;
        let q_4_poly = &self.q_4.0;

        // (a_eval * b_eval * q_m_poly + a_eval * q_l + b_eval * q_r + c_eval
        // * q_o + d_eval * q_4 + q_c) * q_arith_eval
        //
        // a_eval * b_eval * q_m_poly
        let ab = *a_eval * b_eval;
        let a_0 = q_m_poly * &ab;

        // a_eval * q_l
        let a_1 = q_l_poly * a_eval;

        // b_eval * q_r
        let a_2 = q_r_poly * b_eval;

        //c_eval * q_o
        let a_3 = q_o_poly * c_eval;

        // d_eval * q_4
        let a_4 = q_4_poly * d_eval;

        let mut a = a_0 + a_1;
        a = a + a_2;
        a = a + a_3;
        a = a + a_4;
        a = &a + q_c_poly;
        a = &a * q_arith_eval;

        a
    }
}
