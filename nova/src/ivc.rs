use crate::function::FunctionCircuit;
use crate::proof::RecursiveProof;
use crate::{Prover, RelaxedR1cs};
use std::marker::PhantomData;

use crate::circuit::AugmentedFCircuit;
use crate::relaxed_r1cs::{RelaxedR1csInstance, RelaxedR1csWitness};
use zkstd::circuit::prelude::{CircuitDriver, R1cs};
use zkstd::common::{Group, RngCore};
use zkstd::matrix::DenseVectors;

pub struct Ivc<C: CircuitDriver, FC: FunctionCircuit<C>> {
    i: usize,
    z0: DenseVectors<C::Scalar>,
    zi: DenseVectors<C::Scalar>,
    prover: Prover<C>,
    r1cs: R1cs<C>,
    // r1cs instance to be folded
    // u_i
    // represents the correct execution of invocation i of F′
    u_single: RelaxedR1cs<C>,
    // running r1cs instance
    // U_i
    // represents the correct execution of invocations 1, . . . , i - 1 of F′
    u_range: RelaxedR1cs<C>,
    f: PhantomData<FC>,
}

impl<C: CircuitDriver, FC: FunctionCircuit<C>> Ivc<C, FC> {
    pub fn new(rng: impl RngCore, z0: DenseVectors<C::Scalar>) -> Self {
        let mut r1cs = R1cs::default();

        let augmented_circuit = AugmentedFCircuit::<C, FC>::default();
        augmented_circuit.generate(&mut r1cs);

        let prover = Prover::new(r1cs.clone(), rng);
        let mut relaxed_r1cs = RelaxedR1cs::new(r1cs.clone());
        let u_dummy = RelaxedR1csInstance::<C>::dummy(r1cs.l() - 1);
        let w_dummy = RelaxedR1csWitness::<C>::dummy(r1cs.m_l_1(), r1cs.m());
        relaxed_r1cs = relaxed_r1cs.update(&u_dummy, &w_dummy);

        Self {
            i: 0,
            z0: z0.clone(),
            zi: z0,
            prover,
            r1cs,
            u_single: relaxed_r1cs.clone(),
            u_range: relaxed_r1cs,
            f: PhantomData::default(),
        }
    }

    pub fn prove_step(&mut self) -> RecursiveProof<C> {
        let z_next = FC::invoke(&self.zi);
        let (u_range_next, w_range_next, u_single_next_x, commit_t) = if self.i == 0 {
            let u_single_next_x = self.u_range.instance.hash(1, &self.z0, &z_next);
            let (u_range_next, w_range_next, commit_t) = (
                RelaxedR1csInstance::<C>::dummy(1),
                RelaxedR1csWitness::<C>::dummy(self.r1cs.w().len(), self.r1cs.m()),
                C::Affine::ADDITIVE_IDENTITY,
            );

            (u_range_next, w_range_next, u_single_next_x, commit_t)
        } else {
            let (u_range_next, w_range_next, commit_t) =
                self.prover.prove(&self.u_single, &self.u_range);

            let new_instance = RelaxedR1cs::new(self.r1cs.clone());
            new_instance.update(&u_range_next, &w_range_next);
            assert!(new_instance.is_sat());

            let u_single_next_x = u_range_next.hash(self.i + 1, &self.z0, &z_next);

            (u_range_next, w_range_next, u_single_next_x, commit_t)
        };

        let augmented_circuit = AugmentedFCircuit {
            i: self.i,
            z_0: self.z0.clone(),
            z_i: self.zi.clone(),
            u_single: self.u_single.instance.clone(),
            u_range: self.u_range.instance.clone(),
            u_range_next: u_range_next.clone(),
            commit_t,
            f: self.f,
            x: u_single_next_x,
        };

        let mut cs = R1cs::<C>::default();
        augmented_circuit.generate(&mut cs);

        let (u_single_next, w_single_next) = (
            RelaxedR1csInstance::new(DenseVectors::new(cs.x())),
            RelaxedR1csWitness::new(DenseVectors::new(cs.w()), self.r1cs.m()),
        );

        assert_eq!(u_single_next.x.len(), 1);
        assert_eq!(u_single_next.x[0], u_single_next_x);

        self.u_single = self.u_single.update(&u_single_next, &w_single_next);
        self.u_range = self.u_range.update(&u_range_next, &w_range_next);
        self.i += 1;
        self.zi = z_next;

        // ((Ui+1, Wi+1), (ui+1, wi+1))
        let pair = (
            (self.u_range.instance.clone(), self.u_range.witness.clone()),
            (
                self.u_single.instance.clone(),
                self.u_single.witness.clone(),
            ),
        );

        RecursiveProof {
            i: self.i,
            z0: self.z0.clone(),
            zi: self.zi.clone(),
            r1cs: self.r1cs.clone(),
            pair,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Ivc;
    use crate::test::ExampleFunction;

    use crate::RecursiveProof;
    use bn_254::Fr;
    use grumpkin::driver::GrumpkinDriver;
    use rand_core::OsRng;
    use zkstd::circuit::prelude::R1cs;
    use zkstd::matrix::DenseVectors;
    use zkstd::r1cs::test::example_r1cs;

    #[test]
    fn ivc_test() {
        let r1cs: R1cs<GrumpkinDriver> = example_r1cs(1);
        let z0 = DenseVectors::new(vec![Fr::from(3)]);
        let mut ivc = Ivc::<GrumpkinDriver, ExampleFunction<GrumpkinDriver>>::new(OsRng, z0);
        let proof_0 = RecursiveProof {
            i: 0,
            z0: ivc.z0.clone(),
            zi: ivc.zi.clone(),
            r1cs: ivc.r1cs.clone(),
            pair: (
                (ivc.u_range.instance.clone(), ivc.u_range.witness.clone()),
                (ivc.u_single.instance.clone(), ivc.u_single.witness.clone()),
            ),
        };

        assert!(proof_0.verify());

        for i in 0..2 {
            let proof = ivc.prove_step();
            assert!(proof.verify());
        }
    }
}
