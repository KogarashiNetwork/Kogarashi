use crate::relaxed_r1cs::{R1csInstance, R1csWitness, RelaxedR1csInstance, RelaxedR1csWitness};
use std::marker::PhantomData;

use crate::driver::scalar_as_base;
use crate::function::FunctionCircuit;
use crate::ivc::PublicParams;
use zkstd::circuit::prelude::CircuitDriver;
use zkstd::common::{Decode, Encode};
use zkstd::matrix::DenseVectors;

#[derive(Decode, Encode, Clone, Debug, PartialEq, Eq)]
#[allow(clippy::type_complexity)]
pub struct RecursiveProof<E1, E2, FC1, FC2>
where
    E1: CircuitDriver<Base = <E2 as CircuitDriver>::Scalar>,
    E2: CircuitDriver<Base = <E1 as CircuitDriver>::Scalar>,
    FC1: FunctionCircuit<E1::Scalar>,
    FC2: FunctionCircuit<E2::Scalar>,
{
    #[codec(compact)]
    pub i: u64,
    pub(crate) z0_primary: DenseVectors<E1::Scalar>,
    pub(crate) z0_secondary: DenseVectors<E2::Scalar>,
    pub(crate) zi_primary: DenseVectors<E1::Scalar>,
    pub(crate) zi_secondary: DenseVectors<E2::Scalar>,
    pub(crate) instances: (
        // u_single/w_single secondary
        (R1csInstance<E2>, R1csWitness<E2>),
        // u_range/w_range primary
        (RelaxedR1csInstance<E1>, RelaxedR1csWitness<E1>),
        // u_range/w_range secondary
        (RelaxedR1csInstance<E2>, RelaxedR1csWitness<E2>),
    ),
    pub(crate) marker: PhantomData<(FC1, FC2)>,
}

impl<E1, E2, FC1, FC2> RecursiveProof<E1, E2, FC1, FC2>
where
    E1: CircuitDriver<Base = <E2 as CircuitDriver>::Scalar>,
    E2: CircuitDriver<Base = <E1 as CircuitDriver>::Scalar>,
    FC1: FunctionCircuit<E1::Scalar>,
    FC2: FunctionCircuit<E2::Scalar>,
{
    pub fn verify(&self, pp: &PublicParams<E1, E2, FC1, FC2>) -> bool {
        let (
            (u_single_secondary, w_single_secondary),
            (u_range_primary, w_range_primary),
            (u_range_secondary, w_range_secondary),
        ) = self.instances.clone();

        if u_single_secondary.x.len() != 2
            || u_range_primary.x.len() != 2
            || u_range_secondary.x.len() != 2
        {
            return false;
        }
        let (hash_primary, hash_secondary) = {
            (
                u_range_secondary.hash::<E1>(self.i, &self.z0_primary, &self.zi_primary),
                u_range_primary.hash::<E2>(self.i, &self.z0_secondary, &self.zi_secondary),
            )
        };

        if hash_primary != u_single_secondary.x[0]
            || hash_secondary != scalar_as_base::<E2>(u_single_secondary.x[1])
        {
            return false;
        }

        pp.r1cs_shape_primary
            .is_sat_relaxed(&u_range_primary, &w_range_primary)
            && pp
                .r1cs_shape_secondary
                .is_sat_relaxed(&u_range_secondary, &w_range_secondary)
            && pp.r1cs_shape_secondary.is_sat(
                &pp.ck_secondary,
                &u_single_secondary,
                &w_single_secondary,
            )
    }
}
