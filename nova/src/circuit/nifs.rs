use core::marker::PhantomData;
use num_bigint::BigInt;
use num_traits::Num;
use std::ops::Mul;

use crate::driver::{f_to_nat, nat_to_f};
use crate::gadget::{BigNatAssignment, R1csInstanceAssignment, RelaxedR1csInstanceAssignment};
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
        // println!(
        //     "W1 = {:?}, {:?}, {:?}",
        //     u_range.commit_w.get_x().value(cs),
        //     u_range.commit_w.get_y().value(cs),
        //     u_range.commit_w.get_z().value(cs)
        // );
        // println!(
        //     "W2 = {:?}, {:?}, {:?}",
        //     u_single.commit_w.get_x().value(cs),
        //     u_single.commit_w.get_y().value(cs),
        //     u_single.commit_w.get_z().value(cs)
        // );
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

        // u_fold = U.u + r
        let u_fold = &u_range.u + &r;
        FieldAssignment::enforce_eq_constant(cs, &(&(&u_fold - &u_range.u) - &r), &C::Base::zero());

        let r_bn = f_to_nat(&r.value(cs));
        let m_bn = BigInt::from_str_radix(C::ORDER_STR, 16).unwrap();
        let x0_range_bn = f_to_nat(&u_range.x0.value(cs));
        let x1_range_bn = f_to_nat(&u_range.x1.value(cs));
        let x0_single_bn = f_to_nat(&u_single.x0.value(cs));
        let x1_single_bn = f_to_nat(&u_single.x1.value(cs));

        // println!("x1 = {:?}", u_range.x1.value(cs));
        // println!("x2 = {:?}", u_single.x1.value(cs));

        let r_x0 = x0_single_bn.mul(r_bn.clone()) % m_bn.clone();

        // println!("R_x0 = {:?}", nat_to_f::<C::Base>(&r_x0));

        // Fold U.x0 + r * x0
        // let r_x0 = FieldAssignment::mul(cs, &r, &u_single.x0);
        let x0_fold = (x0_range_bn + r_x0) % m_bn.clone();
        // println!("x0_fold = {:?}", nat_to_f::<C::Base>(&x0_fold));

        let r_x1 = x1_single_bn.mul(r_bn) % m_bn.clone();

        // Fold U.x0 + r * x0
        // let r_x0 = FieldAssignment::mul(cs, &r, &u_single.x0);
        let x1_fold = (x1_range_bn + r_x1.clone()) % m_bn;

        // Fold U.x1 + r * x1
        // let r_x1 = FieldAssignment::mul(cs, &r, &u_single.x1);
        // println!("R_x1 = {:?}", nat_to_f::<C::Base>(&r_x1));
        // let x1_fold = &u_range.x1 + &r_x1;
        // println!("x1_fold = {:?}", nat_to_f::<C::Base>(&x1_fold));
        RelaxedR1csInstanceAssignment {
            commit_w: w_fold,
            commit_e: e_fold,
            u: u_fold,
            x0: FieldAssignment::witness(cs, nat_to_f(&x0_fold)),
            x1: FieldAssignment::witness(cs, nat_to_f(&x1_fold)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::driver::{Bn254Driver, GrumpkinDriver};
    use crate::gadget::R1csInstanceAssignment;
    use crate::hash::{MimcRO, MIMC_ROUNDS};
    use crate::prover::tests::example_prover;
    use crate::relaxed_r1cs::{r1cs_instance_and_witness, RelaxedR1csInstance, RelaxedR1csWitness};
    use crate::R1csShape;
    use zkstd::common::CurveGroup;

    use zkstd::r1cs::test::example_r1cs;

    #[test]
    #[ignore]
    fn nifs_circuit() {
        let prover = example_prover();
        let r1cs = example_r1cs::<Bn254Driver>(1);
        let shape = R1csShape::from(r1cs.clone());
        let (x, w) = r1cs_instance_and_witness(&r1cs, &shape, &prover.ck);
        let running_instance = RelaxedR1csInstance::from_r1cs_instance(&prover.ck, &shape, &x);
        let running_witness = RelaxedR1csWitness::from_r1cs_witness(&shape, &w);

        let r1cs_2 = example_r1cs::<Bn254Driver>(2);
        let (instance_to_fold, witness_to_fold) =
            r1cs_instance_and_witness(&r1cs, &shape, &prover.ck);

        let (instance, witness, commit_t) = prover.prove(
            &running_instance,
            &running_witness,
            &instance_to_fold,
            &witness_to_fold,
        );

        let mut transcript = MimcRO::<MIMC_ROUNDS, Bn254Driver>::default();
        transcript.append_point(commit_t);
        running_instance.absorb_by_transcript(&mut transcript);
        let t = prover.compute_cross_term(
            &running_instance,
            &running_witness,
            &instance_to_fold,
            &witness_to_fold,
        );
        let r = transcript.squeeze();

        let mut cs = R1cs::<GrumpkinDriver>::default();
        let r = FieldAssignment::witness(&mut cs, r.into());
        let instance1 = RelaxedR1csInstanceAssignment::witness(&mut cs, &running_instance);
        let instance2 = R1csInstanceAssignment::witness(&mut cs, &instance_to_fold);
        let commit_t = PointAssignment::witness(
            &mut cs,
            commit_t.get_x(),
            commit_t.get_y(),
            commit_t.is_identity(),
        );

        let instance3 = NifsCircuit::verify(&mut cs, r, instance1, instance2, commit_t);
        assert!(cs.is_sat());
    }
}
