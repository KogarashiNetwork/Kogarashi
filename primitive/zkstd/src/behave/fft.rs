// This trait resresents zkSNARKs trait

use core::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use super::field::PrimeField;

use super::{algebra::Field, comp::ParallelCmp};

// TODO: should be right place
pub trait RefOps:
    for<'a> Add<&'a Self, Output = Self>
    + for<'b> Add<&'b Self, Output = Self>
    + for<'a, 'b> Add<&'b Self, Output = Self>
    + for<'a> AddAssign<&'a Self>
    + for<'b> AddAssign<&'b Self>
    + for<'a, 'b> AddAssign<&'b Self>
    + for<'a> Sub<&'a Self, Output = Self>
    + for<'b> Sub<&'b Self, Output = Self>
    + for<'a, 'b> Sub<&'b Self, Output = Self>
    + for<'a> SubAssign<&'a Self>
    + for<'b> SubAssign<&'b Self>
    + for<'a, 'b> SubAssign<&'b Self>
    + for<'a> Mul<&'a Self, Output = Self>
    + for<'b> Mul<&'b Self, Output = Self>
    + for<'a, 'b> Mul<&'b Self, Output = Self>
    + for<'a> MulAssign<&'a Self>
    + for<'b> MulAssign<&'b Self>
    + for<'a, 'b> MulAssign<&'b Self>
{
}

/// This is fft field
/// This is used for fft and has roots of unity
pub trait FftField: PrimeField + ParallelCmp + From<u64> + From<[u64; 4]> + RefOps {
    // factor power of two
    const S: usize;
    // 2^s th root of unity
    const ROOT_OF_UNITY: Self;
    // multiplicative generator
    const MULTIPLICATIVE_GENERATOR: Self;

    fn is_even(&self) -> bool;

    fn pow(self, val: u64) -> Self;

    fn pow_of_2(by: u64) -> Self;

    fn divn(&mut self, n: u32);

    fn mod_2_pow_k(&self, k: u8) -> u8;

    fn mods_2_pow_k(&self, w: u8) -> i8;

    fn mod_by_window(&self, c: usize) -> u64;

    fn from_bytes_wide(bytes: &[u8; 64]) -> Self;

    fn reduce(&self) -> Self;
}

/// This is polynomial
/// This has fft functionality and represents polynomial ring
pub trait Polynomial: Field + ParallelCmp {
    // domain of polynomial
    type Domain: FftField;

    fn evaluate(self, at: Self::Domain) -> Self::Domain;
}
