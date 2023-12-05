use crate::curve::Affine;
use crate::params::PARAM_B3;
use bn_254::{Fq, Fr};
use zkstd::circuit::CircuitDriver;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GrumpkinDriver;

impl CircuitDriver for GrumpkinDriver {
    const NUM_BITS: u16 = 254;

    type Affine = Affine;

    type Base = Fr;

    type Scalar = Fq;

    fn b3() -> Self::Base {
        PARAM_B3
    }
}
