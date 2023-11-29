use crate::function::Function;
use crate::proof::RecursiveProof;
use crate::{Prover, RelaxedR1cs, Verifier};

use crate::relaxed_r1cs::{RelaxedR1csInstance, RelaxedR1csWitness};
use zkstd::circuit::prelude::{CircuitDriver, R1cs};
use zkstd::common::{Group, IntGroup, Ring, RngCore};
use zkstd::matrix::DenseVectors;

pub struct Ivc<C: CircuitDriver> {
    i: usize,
    z0: DenseVectors<C::Scalar>,
    zi: DenseVectors<C::Scalar>,
    prover: Prover<C>,
    r1cs: R1cs<C>,
    // r1cs instance to be folded
    instance: RelaxedR1cs<C>,
    // running r1cs instance
    running_instance: RelaxedR1cs<C>,
}

impl<C: CircuitDriver> Ivc<C> {
    pub fn new(r1cs: R1cs<C>, rng: impl RngCore, z0: DenseVectors<C::Scalar>) -> Self {
        let i = 0;
        let zi = z0.clone();
        let prover = Prover::new(r1cs.clone(), rng);
        let instance = RelaxedR1cs::new(r1cs.clone());
        let running_instance = instance.clone();

        Self {
            i,
            z0,
            zi,
            prover,
            r1cs,
            instance,
            running_instance,
        }
    }

    // augmented function
    pub fn recurse<F: Function<C>>(&mut self) -> C::Scalar {
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
            &self.running_instance.instance
        };

        self.i += 1;
        // zi is z0 for the i == 0
        self.zi = F::invoke(&self.zi);
        hash(self.i, &self.z0, &self.zi, &instance)
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
            prover: _,
            r1cs,
            instance,
            running_instance,
        } = self;

        let ((U_i, W_i), (u_i, w_i)) = p.pair;

        let (U_i_1, W_i_1, commit_T) = if self.i == 0 {
            (
                running_instance.instance.clone(),
                running_instance.witness.clone(),
                running_instance.instance.commit_e.clone(),
            )
        } else {
            let relaxed_r1cs = RelaxedR1cs::new(r1cs.clone());
            let U_relaxed_r1cs = relaxed_r1cs.update(&U_i, &W_i);
            let u_relaxed_r1cs = relaxed_r1cs.update(&u_i, &w_i);
            self.prover.prove(&u_relaxed_r1cs, &U_relaxed_r1cs)
        };

        let (u_i_1, w_i_1) = trace(&U_i, &u_i, self.i, &self.z0, &self.zi, commit_T);

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
        let mut ivc = Ivc::new(r1cs, OsRng, z0);
        let hash = ivc.recurse::<ExampleFunction<GrumpkinDriver>>();
        let proof = ivc.prove(RecursiveProof::default());

        assert!(proof.verify())
    }
}
