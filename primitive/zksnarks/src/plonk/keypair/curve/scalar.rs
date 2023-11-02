use crate::plonk::utils::{check_bit_consistency, extract_bit};
use crate::plonk::Evaluations;
use poly_commit::{Coefficients, Commitment, PointsValue};
use zkstd::common::{vec, Pairing, PrimeField, Ring, Vec};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VerificationKey<P: Pairing> {
    pub q_l: Commitment<P::G1Affine>,
    pub q_r: Commitment<P::G1Affine>,
    pub q_fixed_group_add: Commitment<P::G1Affine>,
}

impl<P: Pairing> VerificationKey<P> {
    pub fn linearize(
        &self,
        ecc_separation_challenge: &P::ScalarField,
        evaluations: &Evaluations<P::ScalarField>,
    ) -> (Vec<P::ScalarField>, Vec<P::G1Affine>) {
        let kappa = ecc_separation_challenge.square();
        let kappa_sq = kappa.square();
        let kappa_cu = kappa_sq * kappa;

        let x_beta_poly = evaluations.q_l_eval;
        let y_beta_eval = evaluations.q_r_eval;

        let acc_x = evaluations.a_eval;
        let acc_x_next = evaluations.a_next_eval;
        let acc_y = evaluations.b_eval;
        let acc_y_next = evaluations.b_next_eval;

        let xy_alpha = evaluations.c_eval;

        let accumulated_bit = evaluations.d_eval;
        let accumulated_bit_next = evaluations.d_next_eval;
        let bit = extract_bit(&accumulated_bit, &accumulated_bit_next);

        // Check bit consistency
        let bit_consistency = check_bit_consistency(bit);

        let y_alpha =
            (bit.square() * (y_beta_eval - P::ScalarField::one())) + P::ScalarField::one();

        let x_alpha = x_beta_poly * bit;

        // xy_alpha consistency check
        let xy_consistency = ((bit * evaluations.q_c_eval) - xy_alpha) * kappa;

        // x accumulator consistency check
        let x_3 = acc_x_next;
        let lhs = x_3 + (x_3 * xy_alpha * acc_x * acc_y * P::PARAM_D);
        let rhs = (x_alpha * acc_y) + (y_alpha * acc_x);
        let x_acc_consistency = (lhs - rhs) * kappa_sq;

        // y accumulator consistency check
        let y_3 = acc_y_next;
        let lhs = y_3 - (y_3 * xy_alpha * acc_x * acc_y * P::PARAM_D);
        let rhs = (x_alpha * acc_x) + (y_alpha * acc_y);
        let y_acc_consistency = (lhs - rhs) * kappa_cu;

        let a = bit_consistency + x_acc_consistency + y_acc_consistency + xy_consistency;

        (
            vec![a * ecc_separation_challenge],
            vec![self.q_fixed_group_add.0],
        )
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ProvingKey<P: Pairing> {
    pub q_l: (Coefficients<P::ScalarField>, PointsValue<P::ScalarField>),
    pub q_r: (Coefficients<P::ScalarField>, PointsValue<P::ScalarField>),
    pub q_c: (Coefficients<P::ScalarField>, PointsValue<P::ScalarField>),
    pub q_fixed_group_add: (Coefficients<P::ScalarField>, PointsValue<P::ScalarField>),
}

impl<P: Pairing> ProvingKey<P> {
    #[allow(clippy::too_many_arguments)]
    pub fn compute_quotient_i(
        &self,
        index: usize,
        ecc_separation_challenge: &P::ScalarField,
        a_w_i: &P::ScalarField,      // acc_x or curr_x
        a_w_i_next: &P::ScalarField, //  // next_x
        b_w_i: &P::ScalarField,      // acc_y or curr_y
        b_w_i_next: &P::ScalarField, // next_y
        c_w_i: &P::ScalarField,      // xy_alpha
        d_w_i: &P::ScalarField,      // accumulated_bit
        d_w_i_next: &P::ScalarField, // accumulated_bit_next
    ) -> P::ScalarField {
        let q_fixed_group_add_i = &self.q_fixed_group_add.1[index];
        let q_c_i = &self.q_c.1[index];

        let kappa = ecc_separation_challenge.square();
        let kappa_sq = kappa.square();
        let kappa_cu = kappa_sq * kappa;

        let x_beta = &self.q_l.1[index];
        let y_beta = &self.q_r.1[index];

        let acc_x = a_w_i;
        let acc_x_next = a_w_i_next;
        let acc_y = b_w_i;
        let acc_y_next = b_w_i_next;

        let xy_alpha = c_w_i;

        let accumulated_bit = d_w_i;
        let accumulated_bit_next = d_w_i_next;
        let bit = extract_bit::<P::ScalarField>(accumulated_bit, accumulated_bit_next);

        // Checks
        //
        // Check bit consistency
        let bit_consistency = check_bit_consistency::<P::ScalarField>(bit);

        // Derive y_alpha and x_alpha from bit
        let y_alpha = bit.square() * (*y_beta - P::ScalarField::one()) + P::ScalarField::one();
        let x_alpha = bit * x_beta;

        // xy_alpha consistency check
        let xy_consistency = ((bit * q_c_i) - xy_alpha) * kappa;

        // x accumulator consistency check
        let x_3 = acc_x_next;
        let lhs: P::ScalarField = *x_3 + (*x_3 * xy_alpha * acc_x * acc_y * P::PARAM_D);
        let rhs = (*acc_x * y_alpha) + (*acc_y * x_alpha);
        let x_acc_consistency = (lhs - rhs) * kappa_sq;

        // y accumulator consistency check
        let y_3 = acc_y_next;
        let lhs: P::ScalarField = *y_3 - (*y_3 * xy_alpha * acc_x * acc_y * P::PARAM_D);
        let rhs = (*acc_y * y_alpha) + (*acc_x * x_alpha);
        let y_acc_consistency = (lhs - rhs) * kappa_cu;

        let identity = bit_consistency + x_acc_consistency + y_acc_consistency + xy_consistency;

        identity * q_fixed_group_add_i * ecc_separation_challenge
    }

    #[allow(clippy::too_many_arguments)]
    pub fn linearize(
        &self,
        ecc_separation_challenge: &P::ScalarField,
        a_eval: &P::ScalarField,
        a_next_eval: &P::ScalarField,
        b_eval: &P::ScalarField,
        b_next_eval: &P::ScalarField,
        c_eval: &P::ScalarField,
        d_eval: &P::ScalarField,
        d_next_eval: &P::ScalarField,
        q_l_eval: &P::ScalarField,
        q_r_eval: &P::ScalarField,
        q_c_eval: &P::ScalarField,
    ) -> Coefficients<P::ScalarField> {
        let q_fixed_group_add_poly = &self.q_fixed_group_add.0;

        let kappa = ecc_separation_challenge.square();
        let kappa_sq = kappa.square();
        let kappa_cu = kappa_sq * kappa;

        let x_beta_eval = q_l_eval;
        let y_beta_eval = q_r_eval;

        let acc_x = a_eval;
        let acc_x_next = a_next_eval;
        let acc_y = b_eval;
        let acc_y_next = b_next_eval;

        let xy_alpha = c_eval;

        let accumulated_bit = d_eval;
        let accumulated_bit_next = d_next_eval;
        let bit = extract_bit::<P::ScalarField>(accumulated_bit, accumulated_bit_next);

        // Check bit consistency
        let bit_consistency = check_bit_consistency::<P::ScalarField>(bit);

        let y_alpha = bit.square() * (*y_beta_eval - P::ScalarField::one()) + P::ScalarField::one();

        let x_alpha = *x_beta_eval * bit;

        // xy_alpha consistency check
        let xy_consistency = ((bit * q_c_eval) - xy_alpha) * kappa;

        // x accumulator consistency check
        let x_3 = acc_x_next;
        let lhs = *x_3 + (*x_3 * xy_alpha * acc_x * acc_y * P::PARAM_D);
        let rhs = (x_alpha * acc_y) + (y_alpha * acc_x);
        let x_acc_consistency = (lhs - rhs) * kappa_sq;

        // y accumulator consistency check
        let y_3 = acc_y_next;
        let lhs = *y_3 - (*y_3 * xy_alpha * acc_x * acc_y * P::PARAM_D);
        let rhs = (x_alpha * acc_x) + (y_alpha * acc_y);
        let y_acc_consistency = (lhs - rhs) * kappa_cu;

        let a = bit_consistency + x_acc_consistency + y_acc_consistency + xy_consistency;

        q_fixed_group_add_poly * &(a * ecc_separation_challenge)
    }
}
