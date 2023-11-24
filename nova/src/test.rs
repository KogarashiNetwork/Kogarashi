use core::marker::PhantomData;

use crate::function::Function;

use r1cs::{CircuitDriver, DenseVectors};

pub(crate) struct ExampleFunction<C: CircuitDriver> {
    mark: PhantomData<C>,
}

impl<C: CircuitDriver> Function<C> for ExampleFunction<C> {
    fn invoke(z: &DenseVectors<C::Scalar>) -> DenseVectors<C::Scalar> {
        let next_z = z[0] * z[0] * z[0] + z[0] + C::Scalar::from(5);
        DenseVectors::new(vec![next_z])
    }
}

#[cfg(test)]
mod tests {
    use crate::hash::{MimcRO, MIMC_ROUNDS};
    use crate::prover::tests::example_prover;
    use crate::relaxed_r1cs::{RelaxedR1csInstance, RelaxedR1csWitness};
    use crate::{RelaxedR1cs, Verifier};
    use bn_254::{Fq, Fr};
    use r1cs::test::example_r1cs;
    use r1cs::{CircuitDriver, DenseVectors};

    /// Verify committed folded instance relations.
    /// Does not open the commitments, but just checks that
    /// the given relaxed instances (instance1, instance2)
    /// when folded result in the folded committed instance values.
    pub fn verify_folded_instance<C: CircuitDriver>(
        r: C::Scalar,
        instance1: &RelaxedR1csInstance<C>,
        instance2: &RelaxedR1csInstance<C>,
        folded_instance: &RelaxedR1csInstance<C>,
        commit_t: &C::Affine,
    ) -> bool {
        let r2 = r * r;

        if folded_instance.commit_e
            != (instance1.commit_e + *commit_t * r + instance2.commit_e * r2).into()
            || folded_instance.u != instance1.u + r * instance2.u
            || folded_instance.commit_w != (instance1.commit_w + instance2.commit_w * r).into()
            || folded_instance.x != &instance1.x + &(&instance2.x * r)
        {
            return false;
        }
        true
    }

    #[test]
    fn nifs_one_fold() {
        let prover = example_prover();
        let mut transcript = MimcRO::<MIMC_ROUNDS, Fq>::default();
        let r1cs_1 = example_r1cs(4);
        let r1cs_2 = example_r1cs(3);

        let mut relaxed_r1cs = RelaxedR1cs::new(r1cs_2);

        let (folded_instance, witness, commit_t) = prover.prove(&r1cs_1, &relaxed_r1cs);
        let verified_instance = Verifier::verify(commit_t, &r1cs_1, &relaxed_r1cs);
        assert_eq!(folded_instance, verified_instance);

        transcript.append_point(commit_t);
        relaxed_r1cs.absorb_by_transcript(&mut transcript);
        let t = prover.compute_cross_term(&r1cs_1, &relaxed_r1cs);
        let r = Fr::from(transcript.squeeze());

        // naive check that the folded witness satisfies the relaxed r1cs
        let z3: Vec<Fr> = [
            vec![verified_instance.u],
            verified_instance.x.get(),
            witness.w.get(),
        ]
        .concat();

        let z1 = [vec![Fr::one()], r1cs_1.x().clone(), r1cs_1.w().clone()].concat();
        let z2 = [
            vec![relaxed_r1cs.instance.u],
            relaxed_r1cs.x().get(),
            relaxed_r1cs.w().get(),
        ]
        .concat();

        let z3_aux: Vec<Fr> = z2
            .iter()
            .map(|x| x * r)
            .zip(z1)
            .map(|(x, y)| x + y)
            .collect();

        assert_eq!(z3, z3_aux);

        // check that relations hold for the 2 inputted instances and the folded one
        let instance1 = RelaxedR1csInstance::default(DenseVectors::new(r1cs_1.x()));
        let instance2 = relaxed_r1cs.instance.clone();
        assert!(relaxed_r1cs.is_sat());
        relaxed_r1cs = relaxed_r1cs.update(
            &instance1,
            &RelaxedR1csWitness::default(DenseVectors::new(r1cs_1.w())),
        );
        assert!(relaxed_r1cs.is_sat());
        relaxed_r1cs = relaxed_r1cs.update(&folded_instance, &witness);
        assert!(relaxed_r1cs.is_sat());

        // next equalities should hold since we started from two cmE of zero-vector E's
        assert_eq!(verified_instance.commit_e, (commit_t * r).into());
        assert_eq!(witness.e, t * r);

        assert!(verify_folded_instance(
            r,
            &instance1,
            &instance2,
            &folded_instance,
            &commit_t
        ));
    }

    #[test]
    fn nifs_fold_loop() {
        let prover = example_prover();
        let r1cs = example_r1cs(1);

        let mut running_r1cs = RelaxedR1cs::new(r1cs);
        assert!(running_r1cs.is_sat());

        for i in 1..10 {
            let incoming_r1cs = example_r1cs(i);

            let (folded_instance, folded_witness, commit_t) =
                prover.prove(&incoming_r1cs, &running_r1cs);
            let verified_instance = Verifier::verify(commit_t, &incoming_r1cs, &running_r1cs);
            assert_eq!(folded_instance, verified_instance);
            running_r1cs = running_r1cs.update(&folded_instance, &folded_witness);
            assert!(running_r1cs.is_sat());
        }
    }
}
