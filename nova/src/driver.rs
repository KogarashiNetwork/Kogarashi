use bn_254::{Fq, Fr, G1Affine as BN254Affine, params::PARAM_B3 as BN254_B3};
use grumpkin::{Affine as GrumpkinAffine, params::PARAM_B3 as Grumpkin_B3};
use zkstd::circuit::CircuitDriver;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GrumpkinDriver;

impl CircuitDriver for GrumpkinDriver {
    const NUM_BITS: u16 = 254;

    type Affine = GrumpkinAffine;

    type Base = Fr;

    type Scalar = Fq;

    fn b3() -> Self::Base {
        Grumpkin_B3
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BN254Driver;

impl CircuitDriver for BN254Driver {
    const NUM_BITS: u16 = 254;

    type Affine = BN254Affine;

    type Base = Fq;

    type Scalar = Fr;

    fn b3() -> Self::Base {
        BN254_B3
    }
}
