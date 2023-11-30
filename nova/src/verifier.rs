use crate::relaxed_r1cs::{RelaxedR1cs, RelaxedR1csInstance};

use crate::hash::{MimcRO, MIMC_ROUNDS};
use core::marker::PhantomData;
use zkstd::circuit::prelude::CircuitDriver;

pub struct Verifier<C: CircuitDriver> {
    mark: PhantomData<C>,
}

impl<C: CircuitDriver> Verifier<C> {
    pub fn verify(
        commit_t: C::Affine,
        r1cs_1: &RelaxedR1cs<C>,
        r1cs_2: &RelaxedR1cs<C>,
    ) -> RelaxedR1csInstance<C> {
        let mut transcript = MimcRO::<MIMC_ROUNDS, C::Base>::default();

        transcript.append_point(commit_t);
        r1cs_2.absorb_by_transcript(&mut transcript);

        let r = transcript.squeeze().into();

        r1cs_2.fold_instance(r1cs_1, r, commit_t)
    }
}

#[cfg(test)]
mod tests {
    use super::{RelaxedR1cs, Verifier};
    use crate::prover::tests::example_prover;

    use zkstd::r1cs::test::example_r1cs;

    #[test]
    fn folding_scheme_verifier_test() {
        let prover = example_prover();
        let r1cs = example_r1cs(1);
        let mut running_r1cs = RelaxedR1cs::new(r1cs);
        for i in 1..10 {
            let r1cs_to_fold = RelaxedR1cs::new(example_r1cs(i));
            let (instance, witness, commit_t) = prover.prove(&r1cs_to_fold, &running_r1cs);
            let verified_instance = Verifier::verify(commit_t, &r1cs_to_fold, &running_r1cs);
            assert_eq!(instance, verified_instance);
            running_r1cs = running_r1cs.update(&instance, &witness);
        }

        assert!(running_r1cs.is_sat())
    }
}
