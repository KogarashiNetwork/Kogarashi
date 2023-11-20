use crate::function::Function;
use crate::proof::RecursiveProof;
use crate::{Prover, RelaxedR1cs};

use r1cs::{CircuitDriver, DenseVectors, R1cs};
use zkstd::common::RngCore;

pub struct Ivc<C: CircuitDriver> {
    i: usize,
    z0: DenseVectors<C::Scalar>,
    zi: DenseVectors<C::Scalar>,
    prover: Prover<C>,
    r1cs: R1cs<C>,
    instance: RelaxedR1cs<C>,
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

    pub fn recurse<F: Function<C>>(&mut self) {
        if self.i == 0 {}
        self.i += 1;
        self.zi = F::invoke(&self.zi);
    }

    pub fn prove(self) -> RecursiveProof<C> {
        let Self {
            i,
            z0,
            zi,
            prover: _,
            r1cs,
            instance,
            running_instance,
        } = self;
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
    use r1cs::{test::example_r1cs, DenseVectors, GrumpkinDriver, R1cs};
    use rand_core::OsRng;

    #[test]
    fn ivc_test() {
        let r1cs: R1cs<GrumpkinDriver> = example_r1cs(1);
        let z0 = DenseVectors::new(r1cs.x());
        let mut ivc = Ivc::new(r1cs, OsRng, z0);
        ivc.recurse::<ExampleFunction<GrumpkinDriver>>();
        let proof = ivc.prove();

        assert!(proof.verify())
    }
}
