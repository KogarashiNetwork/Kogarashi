use crate::params::PARAM_B3;
use bn_254::{Fq, Fr, G1Affine};
use zkstd::circuit::CircuitDriver;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GrumpkinDriver;

impl CircuitDriver for GrumpkinDriver {
    const NUM_BITS: u16 = 254;
    type Affine = G1Affine;

    type Base = Fq;

    type Scalar = Fr;

    fn b3() -> Self::Scalar {
        PARAM_B3
    }
}
