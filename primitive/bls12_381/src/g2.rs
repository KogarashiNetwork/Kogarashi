use crate::fq::Fq;
use crate::fq2::Fq2;
use crate::fr::Fr;
use zero_crypto::arithmetic::bits_384::*;
use zero_crypto::common::*;
use zero_crypto::dress::curve::*;

/// The projective form of coordinate
#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct G2Projective {
    pub(crate) x: Fq2,
    pub(crate) y: Fq2,
    pub(crate) z: Fq2,
}

const IDENTITY: G2Projective = G2Projective {
    x: Fq2::zero(),
    y: Fq2::one(),
    z: Fq2::zero(),
};

const GENERATOR: G2Projective = G2Projective {
    x: Fq2([
        Fq([
            0xf5f28fa202940a10,
            0xb3f5fb2687b4961a,
            0xa1a893b53e2ae580,
            0x9894999d1a3caee9,
            0x6f67b7631863366b,
            0x058191924350bcd7,
        ]),
        Fq([
            0xa5a9c0759e23f606,
            0xaaa0c59dbccd60c3,
            0x3bb17e18e2867806,
            0x1b1ab6cc8541b367,
            0xc2b6ed0ef2158547,
            0x11922a097360edf3,
        ]),
    ]),
    y: Fq2([
        Fq([
            0x4c730af860494c4a,
            0x597cfa1f5e369c5a,
            0xe7e6856caa0a635a,
            0xbbefb5e96e0d495f,
            0x07d3a975f0ef25a2,
            0x0083fd8e7e80dae5,
        ]),
        Fq([
            0xadc0fc92df64b05d,
            0x18aa270a2b1461dc,
            0x86adac6a3be4eba0,
            0x79495c4ec93da33a,
            0xe7175850a43ccaed,
            0x0b2bc2a163de1bf2,
        ]),
    ]),
    z: Fq2::one(),
};

const PARAM_A: Fq2 = Fq2([Fq([0, 0, 0, 0, 0, 0]), Fq([0, 0, 0, 0, 0, 0])]);

const PARAM_B: Fq2 = Fq2([
    Fq([
        0xaa270000000cfff3,
        0x53cc0032fc34000a,
        0x478fe97a6b0a807f,
        0xb1d37ebee6ba24d7,
        0x8ec9733bbf78ab2f,
        0x09d645513d83de7e,
    ]),
    Fq([
        0xaa270000000cfff3,
        0x53cc0032fc34000a,
        0x478fe97a6b0a807f,
        0xb1d37ebee6ba24d7,
        0x8ec9733bbf78ab2f,
        0x09d645513d83de7e,
    ]),
]);

/// The projective form of coordinate
#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct G2Affine {
    x: Fq2,
    y: Fq2,
    is_infinity: bool,
}

curve_operation!(
    Fr,
    Fq2,
    PARAM_A,
    PARAM_B,
    G2Affine,
    G2Projective,
    GENERATOR,
    IDENTITY
);
