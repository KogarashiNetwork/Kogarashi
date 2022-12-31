use crate::fq::Fq;
use crate::fr::Fr;
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

const IDENTITY: G1Projective = G1Projective {
    x: Fq::zero(),
    y: Fq::zero(),
    z: Fq::zero(),
};

const GENERATOR: G1Projective = G1Projective {
    x: Fq([
        0x5cb38790fd530c16,
        0x7817fc679976fff5,
        0x154f95c7143ba1c1,
        0xf0ae6acdf3d0e747,
        0xedce6ecc21dbf440,
        0x120177419e0bfb75,
    ]),
    y: Fq([
        0xbaac93d50ce72271,
        0x8c22631a7918fd8e,
        0xdd595f13570725ce,
        0x51ac582950405194,
        0x0e1c8c3fad0059c0,
        0x0bbc3efc5008a26a,
    ]),
    z: Fq::one(),
};

const PARAM_A: Fq = Fq([0, 0, 0, 0, 0, 0]);

const PARAM_B: Fq = Fq([
    0xaa270000000cfff3,
    0x53cc0032fc34000a,
    0x478fe97a6b0a807f,
    0xb1d37ebee6ba24d7,
    0x8ec9733bbf78ab2f,
    0x09d645513d83de7e,
]);

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
    PARAM_A,
    PARAM_B,
    G1Affine,
    G1Projective,
    GENERATOR,
    IDENTITY
);
