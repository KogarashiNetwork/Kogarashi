use crate::relaxed_r1cs::{R1csInstance, RelaxedR1csInstance};

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
        instance2: &R1csInstance<C>,
    ) -> RelaxedR1csInstance<C> {
        let mut transcript = MimcRO::<MIMC_ROUNDS, C>::default();

        transcript.append_point(commit_t);
        instance1.absorb_by_transcript(&mut transcript);

        let r = transcript.squeeze();

        instance1.fold(instance2, r, commit_t)
    }
}

#[cfg(test)]
mod tests {
    use super::Verifier;
    use crate::prover::tests::example_prover;

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
        let (x, w) = r1cs_instance_and_witness(&r1cs, &shape, &prover.ck);
        let running_instance = RelaxedR1csInstance::from_r1cs_instance(&prover.ck, &shape, &x);
        let running_witness = RelaxedR1csWitness::from_r1cs_witness(&shape, &w);

        for i in 1..10 {
            let r1cs_i = example_r1cs::<GrumpkinDriver>(i);
            let (instance_to_fold, witness_to_fold) =
                r1cs_instance_and_witness(&r1cs_i, &shape, &prover.ck);

            let (instance, witness, commit_t) = prover.prove(
                &running_instance,
                &running_witness,
                &instance_to_fold,
                &witness_to_fold,
            );
            let verified_instance =
                Verifier::verify(commit_t, &running_instance, &instance_to_fold);
            assert_eq!(instance, verified_instance);
            assert!(shape.is_sat_relaxed(&instance, &witness));
        }
    }
}
