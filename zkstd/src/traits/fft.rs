// This trait resresents zkSNARKs trait

use super::field::PrimeField;
use super::{algebra::Field, primitive::ParallelCmp};

// TODO: need to rethink fft and prime field method
/// This is fft field
/// This is used for fft and has roots of unity
pub trait FftField: PrimeField + ParallelCmp + From<[u64; 4]> {
    // factor power of two
    const S: usize;
    // 2^s th root of unity
    const ROOT_OF_UNITY: Self;
    // multiplicative generator
    const MULTIPLICATIVE_GENERATOR: Self;

    fn is_even(&self) -> bool;

    fn pow(self, val: u64) -> Self;

    fn divn(&mut self, n: u32);

    fn mod_2_pow_k(&self, k: u8) -> u8;

    fn mods_2_pow_k(&self, w: u8) -> i8;

    fn reduce(&self) -> Self;

    fn from_hash(hash: &[u8; 64]) -> Self;
}

/// This is polynomial
/// This has fft functionality and represents polynomial ring
pub trait Polynomial: Field + ParallelCmp {
    // domain of polynomial
    type Domain: FftField;

    fn evaluate(self, at: Self::Domain) -> Self::Domain;
}
