use core::marker::PhantomData;

use crate::gadget::RelaxedR1csInstanceAssignment;
use zkstd::circuit::prelude::{CircuitDriver, FieldAssignment, PointAssignment, R1cs};
use zkstd::common::IntGroup;

pub(crate) struct NifsCircuit<C: CircuitDriver> {
    p: PhantomData<C>,
}

impl<C: CircuitDriver> NifsCircuit<C> {
    pub(crate) fn verify<CS: CircuitDriver<Scalar = C::Base>>(
        cs: &mut R1cs<CS>,
        r: FieldAssignment<C::Base>,
        u_single: RelaxedR1csInstanceAssignment<C>,
        u_range: RelaxedR1csInstanceAssignment<C>,
        commit_t: PointAssignment<C::Base>,
    ) -> RelaxedR1csInstanceAssignment<C> {
        let r2 = FieldAssignment::square(cs, &r);
        println!("R2 = {:?}", r2.value(cs));
        // W_fold = U.W + r * u.W
        let r_w = u_range.commit_w.scalar_point(cs, &r);
        let w_fold = u_range.commit_w.add(&r_w, cs);

        // E_fold = u.E + r * T + U.E * r^2
        let r_t = commit_t.scalar_point(cs, &r);
        let r2_e = u_range.commit_e.scalar_point(cs, &r2);
        let e_fold = u_range.commit_e.add(&r_t, cs);
        let e_fold = e_fold.add(&r2_e, cs);

        // u_fold = u.u + r * U.u
        let r_u = FieldAssignment::mul(cs, &r, &u_range.u);
        let u_fold = &u_single.u + &r_u;
        FieldAssignment::enforce_eq_constant(
            cs,
            &(&(&u_fold - &u_single.u) - &r_u),
            &C::Base::zero(),
        );

        // Fold x0 + r * U.x0
        let r_x0 = FieldAssignment::mul(cs, &r, &u_range.x0);
        let x0_fold = &u_single.x0 + &r_x0;

        // Fold x1 + r * U.x1
        let r_x1 = FieldAssignment::mul(cs, &r, &u_range.x1);
        let x1_fold = &u_single.x1 + &r_x1;
        RelaxedR1csInstanceAssignment {
            commit_w: w_fold,
            commit_e: e_fold,
            u: u_fold,
            x0: x0_fold,
            x1: x1_fold,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::driver::{Bn254Driver, GrumpkinDriver};
    use crate::hash::{MimcRO, MIMC_ROUNDS};
    use crate::prover::tests::example_prover;
    use crate::relaxed_r1cs::{RelaxedR1csInstance, RelaxedR1csWitness};
    use zkstd::common::CurveGroup;
    use zkstd::matrix::DenseVectors;
    use zkstd::r1cs::test::example_r1cs;

    #[test]
    fn nifs_circuit() {
        let prover = example_prover();
        let r1cs = example_r1cs::<Bn254Driver>(1);
        let running_instance = RelaxedR1csInstance::new(DenseVectors::new(r1cs.x()));
        let running_witness = RelaxedR1csWitness::new(DenseVectors::new(r1cs.w()), r1cs.m());

        let r1cs_2 = example_r1cs::<Bn254Driver>(2);
        let instance_to_fold = RelaxedR1csInstance::new(DenseVectors::new(r1cs.x()));
        let witness_to_fold = RelaxedR1csWitness::new(DenseVectors::new(r1cs.w()), r1cs.m());
        let (instance, witness, commit_t) = prover.prove(
            &instance_to_fold,
            &witness_to_fold,
            &running_instance,
            &running_witness,
        );

        let mut transcript = MimcRO::<MIMC_ROUNDS, Bn254Driver>::default();
        transcript.append_point(commit_t);
        running_instance.absorb_by_transcript(&mut transcript);
        let t = prover.compute_cross_term(
            &instance_to_fold,
            &witness_to_fold,
            &running_instance,
            &running_witness,
        );
        let r = transcript.squeeze();

        let mut cs = R1cs::<GrumpkinDriver>::default();
        let r = FieldAssignment::witness(&mut cs, r.into());
        let instance1 = RelaxedR1csInstanceAssignment::witness(&mut cs, &instance_to_fold);
        let instance2 = RelaxedR1csInstanceAssignment::witness(&mut cs, &running_instance);
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
