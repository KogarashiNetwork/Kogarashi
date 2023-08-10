use crate::Evaluations;
use poly_commit::Commitment;
use zkstd::common::{vec, Pairing, PrimeField, Ring, TwistedEdwardsCurve, Vec};

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
        let bit = extract_bit::<P::ScalarField>(&accumulated_bit, &accumulated_bit_next);

        // Check bit consistency
        let bit_consistency = check_bit_consistency::<P::ScalarField>(bit);

        let y_alpha =
            (bit.square() * (y_beta_eval - P::ScalarField::one())) + P::ScalarField::one();

        let x_alpha = x_beta_poly * bit;

        // xy_alpha consistency check
        let xy_consistency = ((bit * evaluations.q_c_eval) - xy_alpha) * kappa;

        // x accumulator consistency check
        let x_3 = acc_x_next;
        let lhs = x_3
            + (x_3
                * xy_alpha
                * acc_x
                * acc_y
                * Into::<P::ScalarField>::into(P::JubjubAffine::PARAM_D));
        let rhs = (x_alpha * acc_y) + (y_alpha * acc_x);
        let x_acc_consistency = (lhs - rhs) * kappa_sq;

        // y accumulator consistency check
        let y_3 = acc_y_next;
        let lhs = y_3
            - (y_3
                * xy_alpha
                * acc_x
                * acc_y
                * Into::<P::ScalarField>::into(P::JubjubAffine::PARAM_D));
        let rhs = (x_alpha * acc_x) + (y_alpha * acc_y);
        let y_acc_consistency = (lhs - rhs) * kappa_cu;

        let a = bit_consistency + x_acc_consistency + y_acc_consistency + xy_consistency;

        (
            vec![a * ecc_separation_challenge],
            vec![self.q_fixed_group_add.0],
        )
    }
}

pub(crate) fn extract_bit<F: PrimeField>(curr_acc: &F, next_acc: &F) -> F {
    // Next - 2 * current
    *next_acc - *curr_acc - *curr_acc
}

// Ensures that the bit is either +1, -1 or 0
pub(crate) fn check_bit_consistency<F: PrimeField>(bit: F) -> F {
    let one = F::one();
    bit * (bit - one) * (bit + one)
}
