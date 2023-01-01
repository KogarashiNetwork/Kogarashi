use crate::fq::Fq;
use crate::fr::Fr;
use crate::params::{G1_GENERATOR_X, G1_GENERATOR_Y, G1_PARAM_A, G1_PARAM_B};
use zero_crypto::arithmetic::bits_384::*;
use zero_crypto::common::*;
use zero_crypto::dress::curve::*;

/// The projective form of coordinate
#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct G1Projective {
    pub(crate) x: Fq,
    pub(crate) y: Fq,
    pub(crate) z: Fq,
}

/// The projective form of coordinate
#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct G1Affine {
    pub(crate) x: Fq,
    pub(crate) y: Fq,
    is_infinity: bool,
}

curve_operation!(
    Fr,
    Fq,
    G1_PARAM_A,
    G1_PARAM_B,
    G1Affine,
    G1Projective,
    G1_GENERATOR_X,
    G1_GENERATOR_Y
);

curve_test!(bls12_381, Fr, G1Affine, G1Projective, 100);
