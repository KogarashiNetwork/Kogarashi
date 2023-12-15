use crate::function::FunctionCircuit;
use crate::{PedersenCommitment, Prover, RecursiveProof};
use rand_core::OsRng;
use std::marker::PhantomData;

use crate::circuit::AugmentedFCircuit;
use crate::relaxed_r1cs::{
    r1cs_instance_and_witness, R1csInstance, R1csShape, R1csWitness, RelaxedR1csInstance,
    RelaxedR1csWitness,
};
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
    u_single_secondary: R1csInstance<E2>,
    w_single_secondary: R1csWitness<E2>,
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
    pub fn init(
        pp: &PublicParams<E1, E2, FC1, FC2>,
        z0_primary: DenseVectors<E1::Scalar>,
        z0_secondary: DenseVectors<E2::Scalar>,
    ) -> Self {
        println!("START");
        let mut cs_primary = R1cs::<E1>::default();
        let circuit_primary = AugmentedFCircuit::<E2, FC1> {
            is_primary: true,
            i: 0,
            z_0: z0_primary.clone(),
            z_i: None,
            u_single: None,
            u_range: None,
            commit_t: None,
            f: Default::default(),
        };
        let zi_primary = circuit_primary.generate(&mut cs_primary);

        let (u_single_next_primary, w_single_next_primary) =
            r1cs_instance_and_witness(&cs_primary, &pp.r1cs_shape_primary, &pp.ck_primary);
        assert!(pp.r1cs_shape_primary.is_sat(
            &pp.ck_primary,
            &u_single_next_primary,
            &w_single_next_primary,
        ));
        let prover_primary = Prover::new(pp.r1cs_shape_primary.clone(), pp.ck_primary.clone());

        let mut cs_secondary = R1cs::<E2>::default();
        let circuit_secondary = AugmentedFCircuit::<E1, FC2> {
            is_primary: false,
            i: 0,
            z_0: z0_secondary.clone(),
            z_i: None,
            u_single: Some(u_single_next_primary.clone()),
            u_range: None,
            commit_t: None,
            f: Default::default(),
        };
        let zi_secondary = circuit_secondary.generate(&mut cs_secondary);

        let (u_single_next_secondary, w_single_next_secondary) =
            r1cs_instance_and_witness(&cs_secondary, &pp.r1cs_shape_secondary, &pp.ck_secondary);
        assert!(pp.r1cs_shape_secondary.is_sat(
            &pp.ck_secondary,
            &u_single_next_secondary,
            &w_single_next_secondary,
        ));

        let prover_secondary =
            Prover::new(pp.r1cs_shape_secondary.clone(), pp.ck_secondary.clone());

        let u_dummy = RelaxedR1csInstance::<E2>::dummy(pp.r1cs_shape_secondary.l());
        let w_dummy = RelaxedR1csWitness::<E2>::dummy(
            pp.r1cs_shape_secondary.m_l_1(),
            pp.r1cs_shape_secondary.m(),
        );

        Self {
            i: 0,
            z0_primary,
            z0_secondary,
            zi_primary: DenseVectors::new(
                zi_primary
                    .into_iter()
                    .map(|x| x.value(&cs_primary))
                    .collect(),
            ),
            zi_secondary: DenseVectors::new(
                zi_secondary
                    .into_iter()
                    .map(|x| x.value(&cs_secondary))
                    .collect(),
            ),
            prover_primary,
            prover_secondary,
            u_single_secondary: u_single_next_secondary,
            w_single_secondary: w_single_next_secondary,
            u_range_primary: RelaxedR1csInstance::from_r1cs_instance(
                &pp.ck_primary,
                &pp.r1cs_shape_primary,
                &u_single_next_primary,
            ),
            w_range_primary: RelaxedR1csWitness::from_r1cs_witness(
                &pp.r1cs_shape_primary,
                &w_single_next_primary,
            ),
            u_range_secondary: u_dummy,
            w_range_secondary: w_dummy,
            f: PhantomData::default(),
        }
    }

    pub fn prove_step(
        &mut self,
        pp: &PublicParams<E1, E2, FC1, FC2>,
    ) -> RecursiveProof<E1, E2, FC1, FC2> {
        println!("STEP");
        if self.i == 0 {
            self.i = 1;
            return RecursiveProof {
                i: self.i,
                z0_primary: self.z0_primary.clone(),
                z0_secondary: self.z0_secondary.clone(),
                zi_primary: self.zi_primary.clone(),
                zi_secondary: self.zi_secondary.clone(),
                instances: (
                    (
                        self.u_single_secondary.clone(),
                        self.w_single_secondary.clone(),
                    ),
                    (self.u_range_primary.clone(), self.w_range_primary.clone()),
                    (
                        self.u_range_secondary.clone(),
                        self.w_range_secondary.clone(),
                    ),
                ),
                marker: Default::default(),
            };
        }
        let z_next = FC1::invoke(&self.zi_primary);
        let (u_range_next_secondary, w_range_next_secondary, commit_t_secondary) =
            self.prover_secondary.prove(
                &self.u_range_secondary,
                &self.w_range_secondary,
                &self.u_single_secondary,
                &self.w_single_secondary,
            );

        let mut cs_primary = R1cs::<E1>::default();
        let circuit_primary = AugmentedFCircuit::<E2, FC1> {
            is_primary: true,
            i: self.i,
            z_0: self.z0_primary.clone(),
            z_i: Some(self.zi_primary.clone()),
            u_single: Some(self.u_single_secondary.clone()),
            u_range: Some(self.u_range_secondary.clone()),
            commit_t: Some(commit_t_secondary),
            f: Default::default(),
        };

        let zi_primary = circuit_primary.generate(&mut cs_primary);

        println!("Primary out");
        let (u_single_next_primary, w_single_next_primary) =
            r1cs_instance_and_witness(&cs_primary, &pp.r1cs_shape_primary, &pp.ck_primary);

        let (u_range_next_primary, w_range_next_primary, commit_t_primary) =
            self.prover_primary.prove(
                &self.u_range_primary,
                &self.w_range_primary,
                &u_single_next_primary,
                &w_single_next_primary,
            );

        let mut cs_secondary = R1cs::<E2>::default();
        let circuit_secondary = AugmentedFCircuit::<E1, FC2> {
            is_primary: false,
            i: self.i,
            z_0: self.z0_secondary.clone(),
            z_i: Some(self.zi_secondary.clone()),
            u_single: Some(u_single_next_primary),
            u_range: Some(self.u_range_primary.clone()),
            commit_t: Some(commit_t_primary),
            f: Default::default(),
        };

        let zi_secondary = circuit_secondary.generate(&mut cs_secondary);

        println!("Secondary out");
        let (u_single_next_secondary, w_single_next_secondary) =
            r1cs_instance_and_witness(&cs_secondary, &pp.r1cs_shape_secondary, &pp.ck_secondary);

        // assert!(pp.r1cs_shape_secondary.is_sat(
        //     &pp.ck_secondary,
        //     &u_single_next_secondary,
        //     &w_single_next_secondary,
        // ));

        // update values
        self.i += 1;
        self.u_range_primary = u_range_next_primary;
        self.w_range_primary = w_range_next_primary;
        self.u_range_secondary = u_range_next_secondary;
        self.w_range_secondary = w_range_next_secondary;
        self.u_single_secondary = u_single_next_secondary;
        self.w_single_secondary = w_single_next_secondary;
        self.zi_primary = DenseVectors::new(
            zi_primary
                .into_iter()
                .map(|x| x.value(&cs_primary))
                .collect(),
        );
        self.zi_secondary = DenseVectors::new(
            zi_secondary
                .into_iter()
                .map(|x| x.value(&cs_secondary))
                .collect(),
        );

        RecursiveProof {
            i: self.i,
            z0_primary: self.z0_primary.clone(),
            z0_secondary: self.z0_secondary.clone(),
            zi_primary: self.zi_primary.clone(),
            zi_secondary: self.zi_secondary.clone(),
            instances: (
                (
                    self.u_single_secondary.clone(),
                    self.w_single_secondary.clone(),
                ),
                (self.u_range_primary.clone(), self.w_range_primary.clone()),
                (
                    self.u_range_secondary.clone(),
                    self.w_range_secondary.clone(),
                ),
            ),
            marker: Default::default(),
        }
    }
}

pub struct PublicParams<E1, E2, FC1, FC2>
where
    E1: CircuitDriver<Base = <E2 as CircuitDriver>::Scalar>,
    E2: CircuitDriver<Base = <E1 as CircuitDriver>::Scalar>,
    FC1: FunctionCircuit<E1::Scalar>,
    FC2: FunctionCircuit<E2::Scalar>,
{
    pub r1cs_shape_primary: R1csShape<E1>,
    pub r1cs_shape_secondary: R1csShape<E2>,
    pub ck_primary: PedersenCommitment<E1::Affine>,
    pub ck_secondary: PedersenCommitment<E2::Affine>,
    marker: PhantomData<(FC1, FC2)>,
}

impl<E1, E2, FC1, FC2> PublicParams<E1, E2, FC1, FC2>
where
    E1: CircuitDriver<Base = <E2 as CircuitDriver>::Scalar>,
    E2: CircuitDriver<Base = <E1 as CircuitDriver>::Scalar>,
    FC1: FunctionCircuit<E1::Scalar>,
    FC2: FunctionCircuit<E2::Scalar>,
{
    pub fn setup(rng: OsRng) -> Self {
        // Initialize shape for the primary
        let circuit_primary = AugmentedFCircuit::<E2, FC1> {
            is_primary: true,
            i: 0,
            z_0: DenseVectors::new(vec![E2::Base::zero(); 1]),
            z_i: None,
            u_single: None,
            u_range: None,
            commit_t: None,
            f: Default::default(),
        };
        let mut cs = R1cs::<E1>::default();
        circuit_primary.generate(&mut cs);
        let r1cs_shape_primary = R1csShape::from(cs);

        // Initialize shape for the secondary
        let circuit_secondary = AugmentedFCircuit::<E1, FC2> {
            is_primary: false,
            i: 0,
            z_0: DenseVectors::new(vec![E1::Base::zero(); 1]),
            z_i: None,
            u_single: None,
            u_range: None,
            commit_t: None,
            f: Default::default(),
        };
        let mut cs = R1cs::<E2>::default();
        circuit_secondary.generate(&mut cs);
        let r1cs_shape_secondary = R1csShape::from(cs);

        let k = (r1cs_shape_primary.m().next_power_of_two() as u64).trailing_zeros();
        let ck_primary = PedersenCommitment::<E1::Affine>::new(k.into(), rng);

        let k = (r1cs_shape_secondary.m().next_power_of_two() as u64).trailing_zeros();
        let ck_secondary = PedersenCommitment::<E2::Affine>::new(k.into(), rng);

        PublicParams {
            r1cs_shape_primary,
            r1cs_shape_secondary,
            ck_primary,
            ck_secondary,
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
        >::setup(OsRng);

        let z0_primary = DenseVectors::new(vec![Fr::from(0)]);
        let z0_secondary = DenseVectors::new(vec![Fq::from(0)]);
        let mut ivc =
            Ivc::<Bn254Driver, GrumpkinDriver, ExampleFunction<Fr>, ExampleFunction<Fq>>::init(
                &pp,
                z0_primary,
                z0_secondary,
            );

        for i in 0..2 {
            let proof = ivc.prove_step(&pp);
            assert!(proof.verify(&pp));
        }
    }
}
