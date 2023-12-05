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
    // u
    instance: RelaxedR1cs<C>,
    // running r1cs instance
    // U
    running_instance: RelaxedR1cs<C>,
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
            instance: relaxed_r1cs.clone(),
            running_instance: relaxed_r1cs,
            f: PhantomData::default(),
        }
    }

    pub fn prove_step(&mut self) {
        let z_next = FC::invoke(&self.zi);
        let (u_next, u_next_x, commit_t) = if self.i == 0 {
            let u_next_x = self.running_instance.instance.hash(1, &self.z0, &z_next);
            let (u_next, w_next, commit_t) = (
                RelaxedR1csInstance::<C>::dummy(1),
                RelaxedR1csWitness::<C>::dummy(1, self.r1cs.m()),
                C::Affine::ADDITIVE_IDENTITY,
            );

            (u_next, u_next_x, commit_t)
        } else {
            let (u_next, w_next, commit_t) =
                self.prover.prove(&self.instance, &self.running_instance);

            let new_instance = RelaxedR1cs::new(self.r1cs.clone());
            new_instance.update(&u_next, &w_next);
            assert!(new_instance.is_sat());

            let u_next_x = u_next.hash(self.i + 1, &self.z0, &z_next);

            (u_next, u_next_x, commit_t)
        };

        let augmented_circuit = AugmentedFCircuit {
            i: self.i,
            z_0: self.z0.clone(),
            z_i: self.zi.clone(),
            u_i: self.instance.instance.clone(),
            U_i: self.running_instance.instance.clone(),
            U_i1: u_next,
            commit_t,
            f: self.f,
            x: u_next_x,
        };

        let mut cs = R1cs::<C>::default();
        augmented_circuit.generate(&mut cs);

        let (x_next, w_next) = (cs.x(), cs.w());
        assert_eq!(x_next.len(), 2);
        assert_eq!(x_next[1], u_next_x);

        self.i += 1;
        self.zi = z_next;
    }

    pub fn prove(self, p: RecursiveProof<C>) -> RecursiveProof<C> {
        let Self {
            i,
            z0,
            zi,
            prover,
            r1cs,
            instance,
            running_instance,
            f,
        } = self;

        let ((U_i, W_i), (u_i, w_i)) = p.pair;

        let (U_i_1, W_i_1, commit_t) = if i == 0 {
            (
                running_instance.instance.clone(),
                running_instance.witness.clone(),
                running_instance.instance.commit_e,
            )
        } else {
            let relaxed_r1cs = RelaxedR1cs::new(r1cs.clone());
            let U_relaxed_r1cs = relaxed_r1cs.update(&U_i, &W_i);
            let u_relaxed_r1cs = relaxed_r1cs.update(&u_i, &w_i);
            prover.prove(&u_relaxed_r1cs, &U_relaxed_r1cs)
        };

        let pair = (
            (instance.instance, instance.witness),
            (running_instance.instance, running_instance.witness),
        );

        RecursiveProof {
            i,
            z0,
            zi,
            r1cs,
            pair,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Ivc;
    use crate::test::ExampleFunction;

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
        ivc.prove_step();

        // assert!(proof.verify())
    }
}
