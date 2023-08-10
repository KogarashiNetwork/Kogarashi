use crate::plonk::proof::Evaluations;

use poly_commit::Commitment;
use zkstd::common::{vec, Affine, Vec};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VerificationKey<A: Affine> {
    pub q_m: Commitment<A>,
    pub q_l: Commitment<A>,
    pub q_r: Commitment<A>,
    pub q_o: Commitment<A>,
    pub q_4: Commitment<A>,
    pub q_c: Commitment<A>,
    pub q_arith: Commitment<A>,
}

impl<A: Affine> VerificationKey<A> {
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
