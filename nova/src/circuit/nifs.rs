use core::marker::PhantomData;

use crate::gadget::RelaxedR1csInstanceAssignment;
use zkstd::circuit::prelude::{BinaryAssignment, CircuitDriver, FieldAssignment, R1cs};

pub(crate) struct NifsCircuit<C: CircuitDriver> {
    p: PhantomData<C>,
}

impl<C: CircuitDriver> NifsCircuit<C> {
    pub(crate) fn verify(
        cs: &mut R1cs<C>,
        r: FieldAssignment<C>,
        instance1: RelaxedR1csInstanceAssignment<C>,
        instance2: RelaxedR1csInstanceAssignment<C>,
        instance3: RelaxedR1csInstanceAssignment<C>,
    ) -> BinaryAssignment<C> {
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
            .collect::<Vec<FieldAssignment<C>>>();
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
    use crate::hash::{MimcRO, MIMC_ROUNDS};
    use crate::prover::tests::example_prover;
    use crate::RelaxedR1cs;
    use bn_254::{Fq, Fr};
    use grumpkin::driver::GrumpkinDriver;
    use zkstd::r1cs::test::example_r1cs;

    #[test]
    fn nifs_circuit() {
        let prover = example_prover();
        let r1cs = example_r1cs(1);
        let mut running_r1cs = RelaxedR1cs::new(r1cs);

        let r1cs_to_fold = RelaxedR1cs::new(example_r1cs(2));
        let (instance, witness, commit_t) = prover.prove(&r1cs_to_fold, &running_r1cs);

        let mut transcript = MimcRO::<MIMC_ROUNDS, Fq>::default();
        transcript.append_point(commit_t);
        running_r1cs.absorb_by_transcript(&mut transcript);
        let t = prover.compute_cross_term(&r1cs_to_fold, &running_r1cs);
        let r = Fr::from(transcript.squeeze());

        let mut cs = R1cs::<GrumpkinDriver>::default();
        let r = FieldAssignment::witness(&mut cs, r);
        let instance1 = RelaxedR1csInstanceAssignment::witness(&mut cs, &r1cs_to_fold.instance);
        let instance2 = RelaxedR1csInstanceAssignment::witness(&mut cs, &running_r1cs.instance);
        let instance3 = RelaxedR1csInstanceAssignment::witness(&mut cs, &instance);

        let nifs_check = NifsCircuit::verify(&mut cs, r, instance1, instance2, instance3);
        FieldAssignment::enforce_eq_constant(
            &mut cs,
            &FieldAssignment::from(&nifs_check),
            &Fr::one(),
        );
        assert!(cs.is_sat());
    }
}
