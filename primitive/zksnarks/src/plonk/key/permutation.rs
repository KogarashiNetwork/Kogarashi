use crate::plonk::proof::Evaluations;

use poly_commit::{Coefficients, Commitment, Fft, PointsValue};
use zkstd::common::{vec, CurveAffine, FftField, PrimeField, Vec};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VerificationKey<A: CurveAffine> {
    pub s_sigma_1: Commitment<A>,
    pub s_sigma_2: Commitment<A>,
    pub s_sigma_3: Commitment<A>,
    pub s_sigma_4: Commitment<A>,
}

impl<A: CurveAffine> VerificationKey<A> {
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ProvingKey<F: FftField> {
    pub s_sigma_1: (Coefficients<F>, PointsValue<F>),
    pub s_sigma_2: (Coefficients<F>, PointsValue<F>),
    pub s_sigma_3: (Coefficients<F>, PointsValue<F>),
    pub s_sigma_4: (Coefficients<F>, PointsValue<F>),
    pub linear_evaluations: PointsValue<F>,
    /* Evaluations of f(x) = X
     * [XXX: Remove this and
     * benchmark if it makes a
     * considerable difference
     * -- These are just the
     * domain elements] */
}

impl<F: FftField> ProvingKey<F> {
    const K1: u64 = 7;
    const K2: u64 = 13;
    const K3: u64 = 17;

    #[allow(clippy::too_many_arguments)]
    pub fn compute_quotient_i(
        &self,
        index: usize,
        a_w_i: &F,
        b_w_i: &F,
        c_w_i: &F,
        d_w_i: &F,
        z_i: &F,
        z_i_next: &F,
        alpha: &F,
        l1_alpha_sq: &F,
        beta: &F,
        gamma: &F,
    ) -> F {
        let a = self.compute_quotient_identity_range_check_i(
            index, a_w_i, b_w_i, c_w_i, d_w_i, z_i, alpha, beta, gamma,
        );
        let b = self.compute_quotient_copy_range_check_i(
            index, a_w_i, b_w_i, c_w_i, d_w_i, z_i_next, alpha, beta, gamma,
        );
        let c = self.compute_quotient_term_check_one_i(z_i, l1_alpha_sq);
        a + b + c
    }
    // (a(x) + beta * X + gamma) (b(X) + beta * k1 * X + gamma) (c(X) + beta *
    // k2 * X + gamma)(d(X) + beta * k3 * X + gamma)z(X) * alpha
    #[allow(clippy::too_many_arguments)]
    fn compute_quotient_identity_range_check_i(
        &self,
        index: usize,
        a_w_i: &F,
        b_w_i: &F,
        c_w_i: &F,
        d_w_i: &F,
        z_i: &F,
        alpha: &F,
        beta: &F,
        gamma: &F,
    ) -> F {
        let x = self.linear_evaluations[index];

        (*a_w_i + (*beta * x) + gamma)
            * (*b_w_i + (*beta * F::from(Self::K1) * x) + gamma)
            * (*c_w_i + (*beta * F::from(Self::K2) * x) + gamma)
            * (*d_w_i + (*beta * F::from(Self::K3) * x) + gamma)
            * z_i
            * alpha
    }
    // (a(x) + beta* Sigma1(X) + gamma) (b(X) + beta * Sigma2(X) + gamma) (c(X)
    // + beta * Sigma3(X) + gamma)(d(X) + beta * Sigma4(X) + gamma) Z(X.omega) *
    // alpha
    #[allow(clippy::too_many_arguments)]
    fn compute_quotient_copy_range_check_i(
        &self,
        index: usize,
        a_w_i: &F,
        b_w_i: &F,
        c_w_i: &F,
        d_w_i: &F,
        z_i_next: &F,
        alpha: &F,
        beta: &F,
        gamma: &F,
    ) -> F {
        let s_sigma_1_eval = self.s_sigma_1.1[index];
        let s_sigma_2_eval = self.s_sigma_2.1[index];
        let s_sigma_3_eval = self.s_sigma_3.1[index];
        let s_sigma_4_eval = self.s_sigma_4.1[index];

        let product = (*a_w_i + (*beta * s_sigma_1_eval) + gamma)
            * (*b_w_i + (*beta * s_sigma_2_eval) + gamma)
            * (*c_w_i + (*beta * s_sigma_3_eval) + gamma)
            * (*d_w_i + (*beta * s_sigma_4_eval) + gamma)
            * z_i_next
            * alpha;

        -product
    }
    // L_1(X)[Z(X) - 1]
    fn compute_quotient_term_check_one_i(&self, z_i: &F, l1_alpha_sq: &F) -> F {
        (*z_i - F::one()) * l1_alpha_sq
    }

    pub fn linearize(
        &self,
        z_challenge: &F,
        (alpha, beta, gamma): (&F, &F, &F),
        (a_eval, b_eval, c_eval, d_eval): (&F, &F, &F, &F),
        (sigma_1_eval, sigma_2_eval, sigma_3_eval): (&F, &F, &F),
        z_eval: &F,
        z_poly: &Coefficients<F>,
    ) -> Coefficients<F> {
        let a = self.compute_linearizer_identity_range_check(
            (a_eval, b_eval, c_eval, d_eval),
            z_challenge,
            (alpha, beta, gamma),
            z_poly,
        );
        let b = self.compute_linearizer_copy_range_check(
            (a_eval, b_eval, c_eval),
            z_eval,
            sigma_1_eval,
            sigma_2_eval,
            sigma_3_eval,
            (alpha, beta, gamma),
            &self.s_sigma_4.0,
        );

        // the poly is increased by 2 after blinding it
        let n = (z_poly.degree() - 2).next_power_of_two() as u64;
        let k = n.trailing_zeros();
        let domain = Fft::new(k as usize);
        let c = self.compute_linearizer_check_is_one(&domain, z_challenge, &alpha.square(), z_poly);
        (a + b) + c
    }
    // (a_eval + beta * z_challenge + gamma)(b_eval + beta * K1 * z_challenge +
    // gamma)(c_eval + beta * K2 * z_challenge + gamma) * alpha z(X)
    fn compute_linearizer_identity_range_check(
        &self,
        (a_eval, b_eval, c_eval, d_eval): (&F, &F, &F, &F),
        z_challenge: &F,
        (alpha, beta, gamma): (&F, &F, &F),
        z_poly: &Coefficients<F>,
    ) -> Coefficients<F> {
        let beta_z = *beta * z_challenge;

        // a_eval + beta * z_challenge + gamma
        let mut a_0 = *a_eval + beta_z;
        a_0 += gamma;

        // b_eval + beta * K1 * z_challenge + gamma
        let beta_z_k1 = F::from(Self::K1) * beta_z;
        let mut a_1 = *b_eval + beta_z_k1;
        a_1 += gamma;

        // c_eval + beta * K2 * z_challenge + gamma
        let beta_z_k2 = F::from(Self::K2) * beta_z;
        let mut a_2 = *c_eval + beta_z_k2;
        a_2 += gamma;

        // d_eval + beta * K3 * z_challenge + gamma
        let beta_z_k3 = F::from(Self::K3) * beta_z;
        let mut a_3 = *d_eval + beta_z_k3;
        a_3 += gamma;

        let mut a = a_0 * a_1;
        a *= a_2;
        a *= a_3;
        a *= alpha; // (a_eval + beta * z_challenge + gamma)(b_eval + beta * K1 *
                    // z_challenge + gamma)(c_eval + beta * K2 * z_challenge + gamma)(d_eval
                    // + beta * K3 * z_challenge + gamma) * alpha
        z_poly * &a // (a_eval + beta * z_challenge + gamma)(b_eval + beta * K1
                    // * z_challenge + gamma)(c_eval + beta * K2 * z_challenge +
                    // gamma) * alpha z(X)
    }
    // -(a_eval + beta * sigma_1 + gamma)(b_eval + beta * sigma_2 + gamma)
    // (c_eval + beta * sigma_3 + gamma) * beta *z_eval * alpha^2 * Sigma_4(X)
    #[allow(clippy::too_many_arguments)]
    fn compute_linearizer_copy_range_check(
        &self,
        (a_eval, b_eval, c_eval): (&F, &F, &F),
        z_eval: &F,
        sigma_1_eval: &F,
        sigma_2_eval: &F,
        sigma_3_eval: &F,
        (alpha, beta, gamma): (&F, &F, &F),
        s_sigma_4_poly: &Coefficients<F>,
    ) -> Coefficients<F> {
        // a_eval + beta * sigma_1 + gamma
        let beta_sigma_1 = *beta * sigma_1_eval;
        let mut a_0 = *a_eval + beta_sigma_1;
        a_0 += gamma;

        // b_eval + beta * sigma_2 + gamma
        let beta_sigma_2 = *beta * sigma_2_eval;
        let mut a_1 = *b_eval + beta_sigma_2;
        a_1 += gamma;

        // c_eval + beta * sigma_3 + gamma
        let beta_sigma_3 = *beta * sigma_3_eval;
        let mut a_2 = *c_eval + beta_sigma_3;
        a_2 += gamma;

        let beta_z_eval = *beta * z_eval;

        let mut a = a_0 * a_1 * a_2;
        a *= beta_z_eval;
        a *= alpha; // (a_eval + beta * sigma_1 + gamma)(b_eval + beta * sigma_2 +
                    // gamma)(c_eval + beta * sigma_3 + gamma) * beta * z_eval * alpha

        s_sigma_4_poly * &-a // -(a_eval + beta * sigma_1 + gamma)(b_eval +
                             // beta * sigma_2 + gamma) (c_eval + beta *
                             // sigma_3 + gamma) * beta * z_eval * alpha^2 *
                             // Sigma_4(X)
    }

    fn compute_linearizer_check_is_one(
        &self,
        domain: &Fft<F>,
        z_challenge: &F,
        alpha_sq: &F,
        z_coeffs: &Coefficients<F>,
    ) -> Coefficients<F> {
        // Evaluate l_1(z)
        let l_1_z = domain.evaluate_all_lagrange_coefficients(*z_challenge)[0];

        z_coeffs * &(l_1_z * alpha_sq)
    }
}
