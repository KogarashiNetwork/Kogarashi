use crate::error::Error;
use bn_254::{Fq, Fr, G1Affine};
use grumpkin::params::PARAM_B3 as GRUMPKIN_PARAM_B3;
use zkstd::circuit::CircuitDriver;

use zkstd::circuit::prelude::R1cs;
use zkstd::common::Debug;

/// circuit trait
pub trait Circuit: Default + Debug {
    fn synthesize(&self, constraint_system: &mut R1cs<Bn254Driver>) -> Result<(), Error>;
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Bn254Driver;

impl CircuitDriver for Bn254Driver {
    const ORDER_STR: &'static str =
        "30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001";

    const NUM_BITS: u16 = 254;
    type Affine = G1Affine;

    type Base = Fq;

    type Scalar = Fr;

    fn b3() -> Self::Scalar {
        GRUMPKIN_PARAM_B3
    }
}
