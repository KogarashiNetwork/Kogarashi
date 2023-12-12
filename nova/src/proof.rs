use crate::relaxed_r1cs::{RelaxedR1csInstance, RelaxedR1csWitness};
use std::marker::PhantomData;

use crate::relaxed_r1cs::R1csShape;
use zkstd::circuit::prelude::CircuitDriver;
use zkstd::common::{Group, Ring};
use zkstd::matrix::DenseVectors;

#[allow(clippy::type_complexity)]
pub struct RecursiveProof<E1, E2>
where
    E1: CircuitDriver<Base = <E2 as CircuitDriver>::Scalar>,
    E2: CircuitDriver<Base = <E1 as CircuitDriver>::Scalar>,
{
    pub(crate) i: usize,
    pub(crate) z0: DenseVectors<E1::Scalar>,
    pub(crate) zi: DenseVectors<E1::Scalar>,
    pub(crate) r1cs: R1csShape<E1>,
    pub(crate) pair: (
        // instance-witness pair of instance to be folded
        (RelaxedR1csInstance<E1>, RelaxedR1csWitness<E1>),
        // instance-witness pair of running instance
        (RelaxedR1csInstance<E1>, RelaxedR1csWitness<E1>),
    ),
    pub(crate) marker: PhantomData<E2>,
}

impl<E1, E2> RecursiveProof<E1, E2>
where
    E1: CircuitDriver<Base = <E2 as CircuitDriver>::Scalar>,
    E2: CircuitDriver<Base = <E1 as CircuitDriver>::Scalar>,
{
    pub fn verify(&self) -> bool {
        let Self {
            i,
            z0,
            zi,
            r1cs,
            pair,
            ..
        } = self;
        let ((l_ui, l_wi), (s_ui, s_wi)) = pair;

        if *i == 0 {
            // check if z vector is the same
            z0 == zi
        } else {
            // check that ui.x = hash(vk, i, z0, zi, Ui)
            let expected_x = l_ui.hash::<E2>(*i, z0, zi);
            let check_hash = expected_x == s_ui.x[0].into();

            // check if folded instance has default error vectors and scalar
            let check_defaults =
                s_ui.commit_e == E1::Affine::ADDITIVE_IDENTITY && s_ui.u == E1::Scalar::one();

            // check if instance-witness pair satisfy

            let is_instance_witness_sat =
                self.r1cs.is_sat(&l_ui, &l_wi) && self.r1cs.is_sat(&s_ui, &s_wi);

            check_hash && check_defaults && is_instance_witness_sat
        }
    }
}
