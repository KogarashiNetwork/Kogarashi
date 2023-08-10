use crate::plonk::proof::Evaluations;

use poly_commit::Commitment;
use zkstd::common::{vec, Affine, PrimeField, Vec};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VerificationKey<A: Affine> {
    pub s_sigma_1: Commitment<A>,
    pub s_sigma_2: Commitment<A>,
    pub s_sigma_3: Commitment<A>,
    pub s_sigma_4: Commitment<A>,
}

impl<A: Affine> VerificationKey<A> {
    const K1: u64 = 7;
    const K2: u64 = 13;
    const K3: u64 = 17;

    pub fn linearize(
        &self,
        z_challenge: &A::Scalar,
        (alpha, beta, gamma): (&A::Scalar, &A::Scalar, &A::Scalar),
        l1_eval: &A::Scalar,
        z_comm: A,
        evaluations: &Evaluations<A::Scalar>,
    ) -> (Vec<A::Scalar>, Vec<A>) {
        let alpha_sq = alpha.square();

        // (a_eval + beta * z + gamma)(b_eval + beta * z * k1 +
        // gamma)(c_eval + beta * k2 * z + gamma)(d_eval + beta
        // * k3 * z + gamma) * alpha
        let x = {
            let beta_z = *beta * z_challenge;
            let q_0 = evaluations.a_eval + beta_z + gamma;

            let beta_k1_z = *beta * A::Scalar::from(Self::K1) * z_challenge;
            let q_1 = evaluations.b_eval + beta_k1_z + gamma;

            let beta_k2_z = *beta * A::Scalar::from(Self::K2) * z_challenge;
            let q_2 = evaluations.c_eval + beta_k2_z + gamma;

            let beta_k3_z = *beta * A::Scalar::from(Self::K3) * z_challenge;
            let q_3 = (evaluations.d_eval + beta_k3_z + gamma) * alpha;

            q_0 * q_1 * q_2 * q_3
        };

        // l1(z) * alpha^2
        let r = *l1_eval * alpha_sq;

        // -(a_eval + beta * sigma_1_eval + gamma)(b_eval + beta *
        // sigma_2_eval + gamma)(c_eval + beta * sigma_3_eval +
        // gamma) * alpha^2
        let y = {
            let beta_sigma_1 = *beta * evaluations.s_sigma_1_eval;
            let q_0 = evaluations.a_eval + beta_sigma_1 + gamma;

            let beta_sigma_2 = *beta * evaluations.s_sigma_2_eval;
            let q_1 = evaluations.b_eval + beta_sigma_2 + gamma;

            let beta_sigma_3 = *beta * evaluations.s_sigma_3_eval;
            let q_2 = evaluations.c_eval + beta_sigma_3 + gamma;

            let q_3 = *beta * evaluations.perm_eval * alpha;

            -(q_0 * q_1 * q_2 * q_3)
        };

        (vec![x + r, y], vec![z_comm, self.s_sigma_4.0])
    }
}
