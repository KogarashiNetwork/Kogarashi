use crate::params::PARAM_B3;
use crate::G1Affine;
use crate::{Fq, Fr};
use zkstd::circuit::CircuitDriver;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Bn254Driver;

impl CircuitDriver for Bn254Driver {
    const NUM_BITS: u16 = 254;
    type Affine = G1Affine;

    type Base = Fq;

    type Scalar = Fr;

    fn b3() -> Self::Base {
        PARAM_B3
    }
}
