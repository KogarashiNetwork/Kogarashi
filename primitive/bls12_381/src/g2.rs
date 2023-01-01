use crate::fqn::{Fq12, Fq2};
use crate::fr::Fr;
use crate::params::{BLS_X, G2_GENERATOR_X, G2_GENERATOR_Y, G2_PARAM_A, G2_PARAM_B};
use zero_crypto::arithmetic::bits_384::*;
use zero_crypto::common::*;
use zero_crypto::dress::{curve::*, pairing::bls12_g2_pairing};

/// The projective form of coordinate
#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct G2Projective {
    pub(crate) x: Fq2,
    pub(crate) y: Fq2,
    pub(crate) z: Fq2,
}

/// The projective form of coordinate
#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct G2Affine {
    x: Fq2,
    y: Fq2,
    is_infinity: bool,
}

/// The coefficient for pairing affine format
#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct PairingCoeff(pub(crate) Fq2, pub(crate) Fq2, pub(crate) Fq2);

/// The pairing format coordinate
#[derive(Debug, Clone, Decode, Encode)]
pub struct G2PairingAffine {
    pub coeffs: Vec<PairingCoeff>,
    infinity: bool,
}

curve_operation!(
    Fr,
    Fq2,
    G2_PARAM_A,
    G2_PARAM_B,
    G2Affine,
    G2Projective,
    G2_GENERATOR_X,
    G2_GENERATOR_Y
);
bls12_g2_pairing!(G2Projective, PairingCoeff, G2PairingAffine, Fq12);
curve_test!(bls12_381, Fr, G2Affine, G2Projective, 50);
