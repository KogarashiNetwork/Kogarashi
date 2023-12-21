use core::marker::PhantomData;
use num_bigint::BigInt;
use num_traits::Num;

use crate::gadget::{f_to_nat, BigNatAssignment, BN_LIMB_WIDTH, BN_N_LIMBS};
use crate::gadget::{R1csInstanceAssignment, RelaxedR1csInstanceAssignment};
use zkstd::circuit::prelude::{CircuitDriver, FieldAssignment, PointAssignment, R1cs};
use zkstd::common::{Group, IntGroup};

pub(crate) struct NifsCircuit<C: CircuitDriver> {
    p: PhantomData<C>,
}

impl<C: CircuitDriver> NifsCircuit<C> {
    pub(crate) fn verify<CS: CircuitDriver<Scalar = C::Base>>(
        cs: &mut R1cs<CS>,
        r: FieldAssignment<C::Base>,
        u_range: RelaxedR1csInstanceAssignment<C>,
        u_single: R1csInstanceAssignment<C>,
        commit_t: PointAssignment<C::Base>,
    ) -> RelaxedR1csInstanceAssignment<C> {
        // W_fold = U.W + r * u.W
        let r_w = u_single.commit_w.scalar_point(cs, &r);
        let w_fold = u_range.commit_w.add(&r_w, cs);
        let z_inv = w_fold
            .get_z()
            .value(cs)
            .invert()
            .unwrap_or_else(C::Base::zero);

        // E_fold = U.E + r * T
        let r_t = commit_t.scalar_point(cs, &r);
        let e_fold = u_range.commit_e.add(&r_t, cs);

        let r_bn = f_to_nat(&r.value(cs));
        let m_bn = BigInt::from_str_radix(C::ORDER_STR, 16).unwrap();
        let r_bn_ass =
            BigNatAssignment::witness_from_field_assignment(cs, &r, BN_LIMB_WIDTH, BN_N_LIMBS);
        let m_bn_ass = BigNatAssignment::witness_from_big_int(cs, m_bn, BN_LIMB_WIDTH, BN_N_LIMBS);

        // u_fold = U.u + r
        let u_fold = FieldAssignment::witness(cs, u_range.u.value(cs) + r.value(cs));
        FieldAssignment::enforce_eq_constant(cs, &(&(&u_fold - &u_range.u) - &r), &C::Base::zero());

        // Fold U.x0 + r * x0
        let x0_single_bn = BigNatAssignment::witness_from_big_int(
            cs,
            f_to_nat(&u_single.x0.value(cs)),
            BN_LIMB_WIDTH,
            BN_N_LIMBS,
        );
        let r_x0 = x0_single_bn.mult_mod(cs, &r_bn_ass, &m_bn_ass);
        let x0_fold = u_range.x0.add(&r_x0).red_mod(cs, &m_bn_ass);

        // Fold U.x1 + r * x1
        let x1_single_bn = BigNatAssignment::witness_from_big_int(
            cs,
            f_to_nat(&u_single.x1.value(cs)),
            BN_LIMB_WIDTH,
            BN_N_LIMBS,
        );
        let r_x1 = x1_single_bn.mult_mod(cs, &r_bn_ass, &m_bn_ass);
        let x1_fold = u_range.x1.add(&r_x1).red_mod(cs, &m_bn_ass);

        RelaxedR1csInstanceAssignment {
            commit_w: w_fold,
            commit_e: e_fold,
            u: u_fold,
            x0: x0_fold,
            x1: x1_fold,
        }
    }
}
