use bn_254::{Fq, Fr};
use grumpkin::params::PARAM_B3;
use zkstd::common::PrimeField;

pub trait CircuitDriver: Clone {
    // curve base field
    type Base: PrimeField;

    // curve scalar field
    type Scalar: PrimeField;

    // bn curve 3b param
    fn b3() -> Self::Base;
}

#[derive(Clone, Debug, Default)]
pub struct GrumpkinDriver;

impl CircuitDriver for GrumpkinDriver {
    type Base = Fr;

    type Scalar = Fq;

    fn b3() -> Self::Base {
        PARAM_B3
    }
}
