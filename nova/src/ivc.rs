use crate::function::FunctionCircuit;
use crate::Prover;
use rand_core::OsRng;
use std::marker::PhantomData;

use crate::circuit::AugmentedFCircuit;
use crate::relaxed_r1cs::{R1csShape, RelaxedR1csInstance, RelaxedR1csWitness};
use zkstd::circuit::prelude::{CircuitDriver, R1cs};
use zkstd::common::IntGroup;
use zkstd::matrix::DenseVectors;

pub struct Ivc<E1, E2, FC1, FC2>
where
    E1: CircuitDriver<Base = <E2 as CircuitDriver>::Scalar>,
    E2: CircuitDriver<Base = <E1 as CircuitDriver>::Scalar>,
    FC1: FunctionCircuit<E1::Scalar>,
    FC2: FunctionCircuit<E2::Scalar>,
{
    i: usize,
    z0_primary: DenseVectors<E1::Scalar>,
    z0_secondary: DenseVectors<E2::Scalar>,
    zi_primary: DenseVectors<E1::Scalar>,
    zi_secondary: DenseVectors<E2::Scalar>,
    prover_primary: Prover<E1>,
    prover_secondary: Prover<E2>,
    // r1cs instance to be folded
    // u_i
    // represents the correct execution of invocation i of F′
    u_single_secondary: RelaxedR1csInstance<E2>,
    w_single_secondary: RelaxedR1csWitness<E2>,
    // running r1cs instance
    // U_i
    // represents the correct execution of invocations 1, . . . , i - 1 of F′
    u_range_primary: RelaxedR1csInstance<E1>,
    w_range_primary: RelaxedR1csWitness<E1>,
    u_range_secondary: RelaxedR1csInstance<E2>,
    w_range_secondary: RelaxedR1csWitness<E2>,
    f: PhantomData<(FC1, FC2)>,
}

impl<E1, E2, FC1, FC2> Ivc<E1, E2, FC1, FC2>
where
    E1: CircuitDriver<Base = <E2 as CircuitDriver>::Scalar>,
    E2: CircuitDriver<Base = <E1 as CircuitDriver>::Scalar>,
    FC1: FunctionCircuit<E1::Scalar>,
    FC2: FunctionCircuit<E2::Scalar>,
{
    pub fn new(
        rng: OsRng,
        pp: &PublicParams<E1, E2, FC1, FC2>,
        z0_primary: DenseVectors<E1::Scalar>,
        z0_secondary: DenseVectors<E2::Scalar>,
    ) -> Self {
        let mut cs_primary = R1cs::<E1>::default();
        let circuit_primary = AugmentedFCircuit::<E2, FC1>::default();
        circuit_primary.generate(&mut cs_primary); // get zi_primary

        // get u_single_next/w_single_next primary

        let prover_primary = Prover::new(R1csShape::from(cs_primary.clone()), rng);

        let mut cs_secondary = R1cs::<E2>::default();
        let circuit_secondary = AugmentedFCircuit::<E1, FC2>::default();
        circuit_secondary.generate(&mut cs_secondary); // get zi_secondary

        // get u_single_next/w_single_next secondary

        let prover_secondary = Prover::new(R1csShape::from(cs_secondary.clone()), rng);

        let u_dummy = RelaxedR1csInstance::<E1>::dummy(cs_primary.l() - 1);
        let w_dummy = RelaxedR1csWitness::<E1>::dummy(cs_primary.m_l_1(), cs_primary.m());

        Self {
            i: 0,
            z0_primary,
            z0_secondary,
            zi_primary: Default::default(),
            zi_secondary: Default::default(),
            prover_primary,
            prover_secondary,
            u_single_secondary: RelaxedR1csInstance::dummy(1),
            w_single_secondary: RelaxedR1csWitness::dummy(1, 1),
            u_range_primary: RelaxedR1csInstance::dummy(1),
            w_range_primary: RelaxedR1csWitness::dummy(1, 1),
            u_range_secondary: RelaxedR1csInstance::dummy(1),
            w_range_secondary: RelaxedR1csWitness::dummy(1, 1),
            f: PhantomData::default(),
        }
    }

    pub fn prove_step(&mut self, pp: &PublicParams<E1, E2, FC1, FC2>) {
        //-> RecursiveProof<E1, E2> {
        if self.i == 0 {
            self.i = 1;
            // return
        }
        let z_next = FC1::invoke(&self.zi_primary);
        let (u_range_next_secondary, w_range_next_secondary, commit_t_secondary) =
            self.prover_secondary.prove(
                &self.u_range_secondary,
                &self.w_range_secondary,
                &self.u_single_secondary,
                &self.w_single_secondary,
            );

        let mut cs = R1cs::<E1>::default();
        let circuit_primary = AugmentedFCircuit::<E2, FC1> {
            i: self.i,
            z_0: self.z0_primary.clone(),
            z_i: Some(self.zi_primary.clone()),
            u_single: Some(self.u_single_secondary.clone()),
            u_range: Some(self.u_range_secondary.clone()),
            u_range_next: None,
            commit_t: Some(commit_t_secondary),
            f: Default::default(),
            x: E2::Base::zero(),
        };

        circuit_primary.generate(&mut cs); // zi_primary

        // get u_single_next/w_single_next primary

        let (u_range_next_primary, w_range_next_primary, commit_t_primary) =
            self.prover_primary.prove(
                &self.u_range_primary,
                &self.w_range_primary,
                &self.u_range_primary, // u_single_next_primary
                &self.w_range_primary, // w_single_next_primary
            );

        let mut cs = R1cs::<E2>::default();
        let circuit_primary = AugmentedFCircuit::<E1, FC2> {
            i: self.i,
            z_0: self.z0_secondary.clone(),
            z_i: Some(self.zi_secondary.clone()),
            u_single: Some(self.u_range_primary.clone()), // u_single_next_primary
            u_range: Some(self.u_range_primary.clone()),
            u_range_next: None,
            commit_t: Some(commit_t_primary),
            f: Default::default(),
            x: E1::Base::zero(),
        };

        circuit_primary.generate(&mut cs); // zi_secondary

        // get u_single_next/w_single_next secondary

        // update values
        self.u_range_primary = u_range_next_primary;
        self.w_range_primary = w_range_next_primary;
        self.u_range_secondary = u_range_next_secondary;
        self.w_range_secondary = w_range_next_secondary;
        // self.u_single_secondary = u_single_next_secondary;
        // self.w_single_secondary = w_single_next_secondary;
        self.i += 1;
        // self.zi_primary = zi_primary;
        // self.zi_secondary = zi_secondary;

        // // ((Ui+1, Wi+1), (ui+1, wi+1))
        // let pair = (
        //     (self.u_range.clone(), self.w_range.clone()),
        //     (self.u_single.clone(), self.w_single.clone()),
        // );
        //
        // RecursiveProof {
        //     i: self.i,
        //     z0: self.z0.clone(),
        //     zi: self.zi.clone(),
        //     r1cs: self.r1cs.clone(),
        //     pair,
        //     marker: Default::default(),
        // }
    }
}

pub struct PublicParams<E1, E2, FC1, FC2>
where
    E1: CircuitDriver<Base = <E2 as CircuitDriver>::Scalar>,
    E2: CircuitDriver<Base = <E1 as CircuitDriver>::Scalar>,
    FC1: FunctionCircuit<E1::Scalar>,
    FC2: FunctionCircuit<E2::Scalar>,
{
    r1cs_shape_primary: R1csShape<E1>,
    r1cs_shape_secondary: R1csShape<E2>,
    marker: PhantomData<(FC1, FC2)>,
}

impl<E1, E2, FC1, FC2> PublicParams<E1, E2, FC1, FC2>
where
    E1: CircuitDriver<Base = <E2 as CircuitDriver>::Scalar>,
    E2: CircuitDriver<Base = <E1 as CircuitDriver>::Scalar>,
    FC1: FunctionCircuit<E1::Scalar>,
    FC2: FunctionCircuit<E2::Scalar>,
{
    pub fn setup() -> Self {
        // Initialize shape for the primary
        let circuit_primary = AugmentedFCircuit::<E2, FC1>::default();
        let mut cs = R1cs::<E1>::default();
        circuit_primary.generate(&mut cs);
        let r1cs_shape_primary = R1csShape::from(cs);

        // Initialize shape for the secondary
        let circuit_secondary = AugmentedFCircuit::<E1, FC2>::default();
        let mut cs = R1cs::<E2>::default();
        circuit_secondary.generate(&mut cs);
        let r1cs_shape_secondary = R1csShape::from(cs);

        PublicParams {
            r1cs_shape_primary,
            r1cs_shape_secondary,
            marker: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Ivc, PublicParams};
    use crate::test::ExampleFunction;

    use crate::driver::{Bn254Driver, GrumpkinDriver};
    use bn_254::{Fq, Fr};
    use rand_core::OsRng;
    use zkstd::circuit::prelude::R1cs;
    use zkstd::matrix::DenseVectors;
    use zkstd::r1cs::test::example_r1cs;

    #[test]
    fn ivc_test() {
        let r1cs: R1cs<GrumpkinDriver> = example_r1cs(1);

        // produce public parameters
        let pp = PublicParams::<
            Bn254Driver,
            GrumpkinDriver,
            ExampleFunction<Fr>,
            ExampleFunction<Fq>,
        >::setup();

        let z0_primary = DenseVectors::new(vec![Fr::from(3)]);
        let z0_secondary = DenseVectors::new(vec![Fq::from(3)]);
        let mut ivc =
            Ivc::<Bn254Driver, GrumpkinDriver, ExampleFunction<Fr>, ExampleFunction<Fq>>::new(
                OsRng,
                &pp,
                z0_primary,
                z0_secondary,
            );
        // let proof_0 = RecursiveProof {
        //     i: 0,
        //     z0: ivc.z0.clone(),
        //     zi: ivc.zi.clone(),
        //     r1cs: ivc.r1cs.clone(),
        //     pair: (
        //         (ivc.u_range.clone(), ivc.w_range.clone()),
        //         (ivc.u_single.clone(), ivc.w_single.clone()),
        //     ),
        //     marker: Default::default(),
        // };
        //
        // assert!(proof_0.verify());

        for i in 0..2 {
            ivc.prove_step(&pp);
            // assert!(proof.verify());
        }
    }
}
