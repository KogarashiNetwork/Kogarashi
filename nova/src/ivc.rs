use crate::function::FunctionCircuit;
use crate::proof::RecursiveProof;
use crate::{Prover, RelaxedR1cs, Verifier};
use std::marker::PhantomData;

use crate::circuit::AugmentedFCircuit;
use crate::relaxed_r1cs::{RelaxedR1csInstance, RelaxedR1csWitness};
use zkstd::circuit::prelude::{CircuitDriver, R1cs};
use zkstd::common::{Group, IntGroup, Ring, RngCore};
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
    pub fn new(r1cs: R1cs<C>, rng: impl RngCore, z0: DenseVectors<C::Scalar>) -> Self {
        let i = 0;
        let zi = z0.clone();
        let prover = Prover::new(r1cs.clone(), rng);
        let instance = RelaxedR1cs::new(r1cs.clone());
        let running_instance = instance.clone();

        let mut cs = r1cs.clone();
        let augmented_circuit = AugmentedFCircuit::<C, FC>::default();
        augmented_circuit.generate(&mut cs);

        Self {
            i,
            z0,
            zi,
            prover,
            r1cs,
            instance,
            running_instance,
            f: PhantomData::default(),
        }
    }

    // augmented function
    pub fn recurse<F: FunctionCircuit<C>>(&mut self) -> C::Scalar {
        let instance = if self.i != 0 {
            // check that ui.x = hash(vk, i, z0, zi, Ui), where ui.x is the public IO of ui
            assert_eq!(
                self.running_instance.instance.x,
                DenseVectors::new(vec![hash(
                    self.i,
                    &self.z0,
                    &self.zi,
                    &self.running_instance.instance
                )])
            );

            // check that (ui.E, ui.u) = (u⊥.E, 1)
            assert!(
                self.running_instance.instance.commit_e == C::Affine::ADDITIVE_IDENTITY
                    && self.running_instance.instance.u == C::Scalar::one()
            );

            // compute Ui+1 ← NIFS.V(vk, U, u, T)
            let t = self
                .prover
                .compute_cross_term(&self.instance, &self.running_instance);
            let commit_t = self.prover.pp.commit(&t);
            Verifier::verify(commit_t, &self.instance, &self.running_instance)
        } else {
            // Default instance (dummy)
            self.running_instance.instance.clone()
        };

        self.i += 1;
        // zi is z0 for the i == 0
        self.zi = F::invoke(&self.zi);
        hash(self.i, &self.z0, &self.zi, &instance)
    }

    pub fn prove_step(&mut self) {
        let z_next = FC::invoke(&self.zi);
        let (u_next, u_next_x, commit_t) = if self.i == 0 {
            let u_next_x = self.running_instance.instance.hash(1, &self.z0, &z_next);
            let (u_next, w_next, commit_t) = (
                RelaxedR1csInstance::<C>::default(),
                RelaxedR1csWitness::<C>::default(),
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
            f: self.f.clone(),
            x: u_next_x,
        };

        let mut cs = R1cs::<C>::default();
        augmented_circuit.generate(&mut cs);

        let (x_next, w_next) = (cs.x(), cs.w());
        assert_eq!(x_next.len(), 1);
        assert_eq!(x_next[0], u_next_x);

        self.i += 1;
        self.zi = z_next;
    }

    // pub fn recurse<F: Function<C>>(&mut self) {
    //     if self.i == 0 {}
    //     self.i += 1;
    //     self.zi = F::invoke(&self.zi);
    // }

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

        let (U_i_1, W_i_1, commit_T) = if i == 0 {
            (
                running_instance.instance.clone(),
                running_instance.witness.clone(),
                running_instance.instance.commit_e.clone(),
            )
        } else {
            let relaxed_r1cs = RelaxedR1cs::new(r1cs.clone());
            let U_relaxed_r1cs = relaxed_r1cs.update(&U_i, &W_i);
            let u_relaxed_r1cs = relaxed_r1cs.update(&u_i, &w_i);
            prover.prove(&u_relaxed_r1cs, &U_relaxed_r1cs)
        };

        let (u_i_1, w_i_1) = trace(&U_i, &u_i, i, &z0, &zi, commit_T);

        // Generate p_i+1

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

pub fn hash<C: CircuitDriver>(
    i: usize,
    z0: &DenseVectors<C::Scalar>,
    zi: &DenseVectors<C::Scalar>,
    u: &RelaxedR1csInstance<C>,
) -> C::Scalar {
    // MIMC
    C::Scalar::zero()
}

pub fn trace<C: CircuitDriver>(
    to_fold: &RelaxedR1csInstance<C>,
    running: &RelaxedR1csInstance<C>,
    i: usize,
    z0: &DenseVectors<C::Scalar>,
    zi: &DenseVectors<C::Scalar>,
    commit_T: C::Affine,
) -> (RelaxedR1csInstance<C>, RelaxedR1csWitness<C>) {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::Ivc;
    use crate::test::ExampleFunction;

    use crate::RecursiveProof;
    use grumpkin::driver::GrumpkinDriver;
    use rand_core::OsRng;
    use zkstd::circuit::prelude::R1cs;
    use zkstd::matrix::DenseVectors;
    use zkstd::r1cs::test::example_r1cs;

    #[test]
    fn ivc_test() {
        let r1cs: R1cs<GrumpkinDriver> = example_r1cs(1);
        let z0 = DenseVectors::new(r1cs.x());
        let mut ivc = Ivc::<GrumpkinDriver, ExampleFunction<GrumpkinDriver>>::new(r1cs, OsRng, z0);
        let hash = ivc.recurse::<ExampleFunction<GrumpkinDriver>>();
        let proof = ivc.prove(RecursiveProof::default());

        assert!(proof.verify())
    }
}
