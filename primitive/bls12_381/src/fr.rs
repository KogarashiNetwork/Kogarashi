use zero_crypto::arithmetic::bits_256::*;
use zero_crypto::common::*;
use zero_crypto::dress::field::*;

#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct Fr(pub(crate) [u64; 4]);

const MODULUS: [u64; 4] = [
    0xffffffff00000001,
    0x53bda402fffe5bfe,
    0x3339d80809a1d805,
    0x73eda753299d7d48,
];

const GENERATOR: [u64; 4] = [
    0x0000000efffffff1,
    0x17e363d300189c0f,
    0xff9c57876f8457b0,
    0x351332208fc5a8c4,
];

const IDENTITY: [u64; 4] = [1, 0, 0, 0];

/// R = 2^256 mod r
const R: [u64; 4] = [
    0x00000001fffffffe,
    0x5884b7fa00034802,
    0x998c4fefecbc4ff5,
    0x1824b159acc5056f,
];

/// R^2 = 2^512 mod r
const R2: [u64; 4] = [
    0xc999e990f3f29c6d,
    0x2b6cedcb87925c23,
    0x05d314967254398f,
    0x0748d9d99f59ff11,
];

/// R^3 = 2^768 mod r
const R3: [u64; 4] = [
    0xc62c1807439b73af,
    0x1b3e0d188cf06990,
    0x73d13c71c7b5f418,
    0x6e2a5bb9c8db33e9,
];

pub(crate) const INV: u64 = 0xfffffffeffffffff;

const S: usize = 32;

const ROOT_OF_UNITY: Fr = Fr([
    0xaa9f02ab1d6124de,
    0xb3524a6466112932,
    0x7342261215ac260b,
    0x4d6b87b1da259e2,
]);

fft_field_operation!(
    Fr,
    MODULUS,
    GENERATOR,
    IDENTITY,
    INV,
    ROOT_OF_UNITY,
    R2,
    R3,
    S
);

impl Fr {
    pub(crate) const fn zero() -> Self {
        Self(zero())
    }

    pub(crate) const fn one() -> Self {
        Self(R)
    }

    pub(crate) const fn to_mont_form(val: [u64; 4]) -> Self {
        Self(to_mont_form(val, R2, MODULUS, INV))
    }
}
