mod basic;
mod field;
mod group;
mod ring;

pub use basic::*;
pub use field::*;
pub use group::*;
pub use ring::*;

use crate::arithmetic::{add, double, invert, mul, neg, square, sub};
use crate::behave::{Basic, FftField, Field, Group, ParityCmp, PrimeField, Ring};
use core::{
    cmp::Ordering,
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Add, Mul, Neg, Sub},
    ops::{AddAssign, MulAssign, SubAssign},
};
use parity_scale_codec::{Decode, Encode};

pub(crate) const MODULUS: Fr = Fr([
    0xd0970e5ed6f72cb7,
    0xa6682093ccc81082,
    0x06673b0101343b00,
    0x0e7db4ea6533afa9,
]);

pub(crate) const GENERATOR: Fr = Fr([2, 0, 0, 0]);

pub(crate) const IDENTITY: Fr = Fr([1, 0, 0, 0]);

/// R = 2^256 mod r
const R: [u64; 4] = [
    0x25f80bb3b99607d9,
    0xf315d62f66b6e750,
    0x932514eeeb8814f4,
    0x09a6fc6f479155c6,
];

/// R^2 = 2^512 mod r
const R2: &[u64; 4] = &[
    0x67719aa495e57731,
    0x51b0cef09ce3fc26,
    0x69dab7fac026e9a5,
    0x04f6547b8d127688,
];

/// R^3 = 2^768 mod r
const R3: &[u64; 4] = &[
    0xe0d6c6563d830544,
    0x323e3883598d0f85,
    0xf0fea3004c2e2ba8,
    0x05874f84946737ec,
];

pub(crate) const INV: u64 = 0x1ba3a358ef788ef9;

const S: u32 = 1;

const ROOT_OF_UNITY: Fr = Fr([
    0xaa9f02ab1d6124de,
    0xb3524a6466112932,
    0x7342261215ac260b,
    0x4d6b87b1da259e2,
]);

#[derive(Clone, Copy, Debug, Decode, Encode)]
pub struct Fr(pub(crate) [u64; 4]);

fft_field_operation!(Fr, MODULUS, GENERATOR, IDENTITY, INV, ROOT_OF_UNITY);