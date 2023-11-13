use bn_254::{Fq, Fr, G1Affine};
use grumpkin::params::PARAM_B3;
use zkstd::common::{BNAffine, PrimeField};

pub trait CircuitDriver: Clone {
    // curve affine
    type Affine: BNAffine<Scalar = Self::Scalar, Base = Self::Base>;

    // curve base field
    type Base: PrimeField;

    // curve scalar field
    type Scalar: PrimeField + From<Self::Base> + ff::PrimeField;

    // bn curve 3b param
    fn b3() -> Self::Scalar;
}

#[derive(Clone, Debug, Default)]
pub struct GrumpkinDriver;

impl CircuitDriver for GrumpkinDriver {
    type Affine = G1Affine;

    type Base = Fq;

    type Scalar = Fr;

    fn b3() -> Self::Scalar {
        PARAM_B3
    }
}
