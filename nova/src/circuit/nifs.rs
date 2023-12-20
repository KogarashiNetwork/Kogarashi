use core::marker::PhantomData;
use num_bigint::BigInt;
use num_traits::Num;
use std::ops::{Add, Mul};

use crate::gadget::{f_to_nat, nat_to_f};
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

        // TODO: Should be done without using BigInt
        // u_fold = U.u + r
        let u = f_to_nat(&u_range.u.value(cs));
        let u_fold = FieldAssignment::witness(cs, nat_to_f(&(u.add(r_bn.clone()) % m_bn.clone())));
        // FieldAssignment::enforce_eq_constant(cs, &(&(&u_fold - &u_range.u) - &r), &C::Base::zero());

        // TODO: BigNatAssignment should be use for module arithmetics
        // Fold U.x0 + r * x0
        let x0_range_bn = f_to_nat(&u_range.x0.value(cs));
        let x0_single_bn = f_to_nat(&u_single.x0.value(cs));
        let r_x0 = x0_single_bn.mul(r_bn.clone()) % m_bn.clone();
        let x0_fold = (x0_range_bn + r_x0) % m_bn.clone();
        // Fold U.x1 + r * x1
        let x1_range_bn = f_to_nat(&u_range.x1.value(cs));
        let x1_single_bn = f_to_nat(&u_single.x1.value(cs));
        let r_x1 = x1_single_bn.mul(r_bn) % m_bn.clone();
        let x1_fold = (x1_range_bn + r_x1) % m_bn;

        RelaxedR1csInstanceAssignment {
            commit_w: w_fold,
            commit_e: e_fold,
            u: u_fold,
            x0: FieldAssignment::witness(cs, nat_to_f(&x0_fold)),
            x1: FieldAssignment::witness(cs, nat_to_f(&x1_fold)),
        }
    }
}
