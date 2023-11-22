use crate::relaxed_r1cs::{RelaxedR1cs, RelaxedR1csInstance};

use crate::hash::MimcRO;
use core::marker::PhantomData;
use r1cs::{CircuitDriver, R1cs};

pub struct Verifier<C: CircuitDriver> {
    mark: PhantomData<C>,
}

impl<C: CircuitDriver> Verifier<C> {
    pub fn verify(
        commit_t: C::Affine,
        r1cs: &R1cs<C>,
        relaxed_r1cs: &RelaxedR1cs<C>,
    ) -> RelaxedR1csInstance<C> {
        let mut transcript = MimcRO::<322, C::Base>::default();

        transcript.append_point(commit_t);
        relaxed_r1cs.absorb_by_transcript(&mut transcript);

        let r = transcript.squeeze().into();

        relaxed_r1cs.fold_instance(r1cs, r, commit_t)
    }
}

#[cfg(test)]
mod tests {
    use super::{RelaxedR1cs, Verifier};
    use crate::prover::tests::example_prover;

    use r1cs::test::example_r1cs;

    #[test]
    fn folding_scheme_verifier_test() {
        let prover = example_prover();
        let r1cs = example_r1cs(1);
        let mut relaxed_r1cs = RelaxedR1cs::new(r1cs);
        for i in 1..10 {
            let r1cs = example_r1cs(i);
            let (instance, witness, commit_t) = prover.prove(&r1cs, &relaxed_r1cs);
            let verified_instance = Verifier::verify(commit_t, &r1cs, &relaxed_r1cs);
            assert_eq!(instance, verified_instance);
            relaxed_r1cs = relaxed_r1cs.update(&instance, &witness);
        }

        assert!(relaxed_r1cs.is_sat())
    }
}
