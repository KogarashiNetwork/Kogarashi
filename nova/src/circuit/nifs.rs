use core::marker::PhantomData;

use crate::gadget::RelaxedR1csInstanceAssignment;
use zkstd::circuit::prelude::{BinaryAssignment, CircuitDriver, FieldAssignment, R1cs};

pub(crate) struct NifsCircuit<C: CircuitDriver> {
    p: PhantomData<C>,
}

impl<C: CircuitDriver> NifsCircuit<C> {
    pub(crate) fn verify<CS: CircuitDriver<Scalar = C::Base>>(
        cs: &mut R1cs<CS>,
        r: FieldAssignment<C::Base>,
        instance1: RelaxedR1csInstanceAssignment<C>,
        instance2: RelaxedR1csInstanceAssignment<C>,
        instance3: RelaxedR1csInstanceAssignment<C>,
    ) -> BinaryAssignment {
        let r_u = FieldAssignment::mul(cs, &r, &instance2.u);
        let first_check = FieldAssignment::is_eq(cs, &instance3.u, &(&instance1.u + &r_u));

        let x = instance1
            .x
            .iter()
            .zip(instance2.x)
            .map(|(x1, x2)| {
                let r_x2 = FieldAssignment::mul(cs, &r, &x2);
                x1 + &r_x2
            })
            .collect::<Vec<FieldAssignment<C::Base>>>();
        let second_check =
            x.iter()
                .zip(instance3.x)
                .fold(BinaryAssignment::witness(cs, 1), |acc, (a, b)| {
                    let check = FieldAssignment::is_eq(cs, a, &b);
                    BinaryAssignment::and(cs, &acc, &check)
                });
        BinaryAssignment::and(cs, &first_check, &second_check)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::driver::{Bn254Driver, GrumpkinDriver};
    use crate::hash::{MimcRO, MIMC_ROUNDS};
    use crate::prover::tests::example_prover;
    use crate::relaxed_r1cs::{RelaxedR1csInstance, RelaxedR1csWitness};
    use bn_254::Fq;
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
        let instance3 = RelaxedR1csInstanceAssignment::witness(&mut cs, &instance);

        let nifs_check = NifsCircuit::verify(&mut cs, r, instance1, instance2, instance3);
        FieldAssignment::enforce_eq_constant(
            &mut cs,
            &FieldAssignment::from(&nifs_check),
            &Fq::one(),
        );
        assert!(cs.is_sat());
    }
}
