use crate::function::FunctionCircuit;
use crate::proof::RecursiveProof;
use crate::Prover;
use std::marker::PhantomData;

use crate::circuit::AugmentedFCircuit;
use crate::relaxed_r1cs::{R1csShape, RelaxedR1csInstance, RelaxedR1csWitness};
use zkstd::circuit::prelude::{CircuitDriver, R1cs};
use zkstd::common::{Group, RngCore};
use zkstd::matrix::DenseVectors;

pub struct Ivc<E1, E2, FC>
where
    E1: CircuitDriver<Base = <E2 as CircuitDriver>::Scalar>,
    E2: CircuitDriver<Base = <E1 as CircuitDriver>::Scalar>,
    FC: FunctionCircuit<E1::Scalar>,
{
    i: usize,
    z0: DenseVectors<E1::Scalar>,
    zi: DenseVectors<E1::Scalar>,
    prover: Prover<E1>,
    r1cs: R1csShape<E1>,
    // r1cs instance to be folded
    // u_i
    // represents the correct execution of invocation i of F′
    u_single: RelaxedR1csInstance<E1>,
    w_single: RelaxedR1csWitness<E1>,
    // running r1cs instance
    // U_i
    // represents the correct execution of invocations 1, . . . , i - 1 of F′
    u_range: RelaxedR1csInstance<E1>,
    w_range: RelaxedR1csWitness<E1>,
    f: PhantomData<(FC, E2)>,
}

impl<E1, E2, FC> Ivc<E1, E2, FC>
where
    E1: CircuitDriver<Base = <E2 as CircuitDriver>::Scalar>,
    E2: CircuitDriver<Base = <E1 as CircuitDriver>::Scalar>,
    FC: FunctionCircuit<E1::Scalar>,
{
    pub fn new(rng: impl RngCore, z0: DenseVectors<E1::Scalar>) -> Self {
        let mut r1cs = R1cs::<E1>::default();

        let augmented_circuit = AugmentedFCircuit::<E2, FC>::default();
        augmented_circuit.generate(&mut r1cs);

        let prover = Prover::new(R1csShape::from(r1cs.clone()), rng);
        let u_dummy = RelaxedR1csInstance::<E1>::dummy(r1cs.l() - 1);
        let w_dummy = RelaxedR1csWitness::<E1>::dummy(r1cs.m_l_1(), r1cs.m());

        Self {
            i: 0,
            z0: z0.clone(),
            zi: z0,
            prover,
            r1cs: R1csShape::from(r1cs),
            u_single: u_dummy.clone(),
            w_single: w_dummy.clone(),
            u_range: u_dummy,
            w_range: w_dummy,
            f: PhantomData::default(),
        }
    }

    pub fn prove_step(&mut self) -> RecursiveProof<E1, E2> {
        let z_next = FC::invoke(&self.zi);
        let (u_range_next, w_range_next, u_single_next_x, commit_t) = if self.i == 0 {
            let u_single_next_x = self.u_range.hash(1, &self.z0, &z_next);
            let (u_range_next, w_range_next, commit_t) = (
                RelaxedR1csInstance::<E1>::dummy(1),
                RelaxedR1csWitness::<E1>::dummy(self.r1cs.l(), self.r1cs.m()),
                E1::Affine::ADDITIVE_IDENTITY,
            );

            (u_range_next, w_range_next, u_single_next_x, commit_t)
        } else {
            let (u_range_next, w_range_next, commit_t) =
                self.prover
                    .prove(&self.u_single, &self.w_single, &self.u_range, &self.w_range);

            assert!(self.r1cs.is_sat(&u_range_next, &w_range_next));

            let u_single_next_x = u_range_next.hash(self.i + 1, &self.z0, &z_next);

            (u_range_next, w_range_next, u_single_next_x, commit_t)
        };

        let augmented_circuit = AugmentedFCircuit::<E2, FC> {
            i: self.i,
            z_0: self.z0.clone(),
            z_i: self.zi.clone(),
            u_single: self.u_single.clone(),
            u_range: self.u_range.clone(),
            u_range_next: u_range_next.clone(),
            commit_t,
            f: self.f,
            x: u_single_next_x,
        };

        let mut cs = R1cs::<E1>::default();
        augmented_circuit.generate(&mut cs);

        let (u_single_next, w_single_next) = (
            RelaxedR1csInstance::new(DenseVectors::new(cs.x())),
            RelaxedR1csWitness::new(DenseVectors::new(cs.w()), self.r1cs.m()),
        );

        assert_eq!(u_single_next.x.len(), 1);
        assert_eq!(u_single_next.x[0], u_single_next_x);

        self.u_single = u_single_next;
        self.w_single = w_single_next;
        self.u_range = u_range_next;
        self.w_range = w_range_next;
        self.i += 1;
        self.zi = z_next;

        // ((Ui+1, Wi+1), (ui+1, wi+1))
        let pair = (
            (self.u_range.clone(), self.w_range.clone()),
            (self.u_single.clone(), self.w_single.clone()),
        );

        RecursiveProof {
            i: self.i,
            z0: self.z0.clone(),
            zi: self.zi.clone(),
            r1cs: self.r1cs.clone(),
            pair,
            marker: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Ivc;
    use crate::test::ExampleFunction;

    use crate::driver::{Bn254Driver, GrumpkinDriver};
    use crate::RecursiveProof;
    use bn_254::Fr;
    use rand_core::OsRng;
    use zkstd::circuit::prelude::R1cs;
    use zkstd::matrix::DenseVectors;
    use zkstd::r1cs::test::example_r1cs;

    #[test]
    fn ivc_test() {
        let r1cs: R1cs<GrumpkinDriver> = example_r1cs(1);
        let z0 = DenseVectors::new(vec![Fr::from(3)]);
        let mut ivc = Ivc::<Bn254Driver, GrumpkinDriver, ExampleFunction<Fr>>::new(OsRng, z0);
        let proof_0 = RecursiveProof {
            i: 0,
            z0: ivc.z0.clone(),
            zi: ivc.zi.clone(),
            r1cs: ivc.r1cs.clone(),
            pair: (
                (ivc.u_range.clone(), ivc.w_range.clone()),
                (ivc.u_single.clone(), ivc.w_single.clone()),
            ),
            marker: Default::default(),
        };

        assert!(proof_0.verify());

        for i in 0..2 {
            let proof = ivc.prove_step();
            assert!(proof.verify());
        }
    }
}
