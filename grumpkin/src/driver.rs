use crate::params::PARAM_B;
use bn_254::{Fq, Fr};
use r1cs::CircuitDriver;

pub struct GrumpkinDriver {}

impl CircuitDriver for GrumpkinDriver {
    type Base = Fr;

    type Scalar = Fq;

    fn b() -> Self::Base {
        PARAM_B
    }
}
