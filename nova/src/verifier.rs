use crate::relaxed_r1cs::RelaxedR1csInstance;

use crate::hash::{MimcRO, MIMC_ROUNDS};
use core::marker::PhantomData;
use zkstd::circuit::prelude::CircuitDriver;

pub struct Verifier<C: CircuitDriver> {
    mark: PhantomData<C>,
}

impl<C: CircuitDriver> Verifier<C> {
    pub fn verify(
        commit_t: C::Affine,
        instance1: &RelaxedR1csInstance<C>,
        instance2: &RelaxedR1csInstance<C>,
    ) -> RelaxedR1csInstance<C> {
        let mut transcript = MimcRO::<MIMC_ROUNDS, C>::default();

        transcript.append_point(commit_t);
        instance2.absorb_by_transcript(&mut transcript);

        let r = transcript.squeeze();

        instance2.fold(instance1, r, commit_t)
    }
}

#[cfg(test)]
mod tests {
    use super::Verifier;
    use crate::prover::tests::example_prover;
    use zkstd::matrix::DenseVectors;

    use crate::driver::GrumpkinDriver;
    use crate::relaxed_r1cs::{
        r1cs_instance_and_witness, R1csShape, RelaxedR1csInstance, RelaxedR1csWitness,
    };
    use zkstd::r1cs::test::example_r1cs;

    #[test]
    fn recursive_nifs_test() {
        let prover = example_prover();
        let r1cs = example_r1cs::<GrumpkinDriver>(1);
        let shape = R1csShape::from(r1cs.clone());
        let (x, w) = r1cs_instance_and_witness(&r1cs, &shape);
        let running_instance = RelaxedR1csInstance::new(DenseVectors::new(x));
        let running_witness = RelaxedR1csWitness::new(DenseVectors::new(w), shape.m());

        for i in 1..10 {
            let r1cs_i = example_r1cs::<GrumpkinDriver>(i);
            let (x, w) = r1cs_instance_and_witness(&r1cs_i, &shape);
            let instance_to_fold = RelaxedR1csInstance::new(DenseVectors::new(x));
            let witness_to_fold = RelaxedR1csWitness::new(DenseVectors::new(w), shape.m());

            let (instance, witness, commit_t) = prover.prove(
                &instance_to_fold,
                &witness_to_fold,
                &running_instance,
                &running_witness,
            );
            let verified_instance =
                Verifier::verify(commit_t, &instance_to_fold, &running_instance);
            assert_eq!(instance, verified_instance);
            assert!(shape.is_sat(&instance, &witness));
        }
    }
}
