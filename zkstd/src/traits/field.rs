// This trait resresents prime field

use super::{algebra::Field, primitive::ParityCmp};
use crate::arithmetic::utils::{Bits, Nafs};

use core::{
    fmt::Debug,
    ops::{BitAnd, BitXor},
};
use sp_std::vec::Vec;

/// This is prime field trait
pub trait PrimeField: Field + ParityCmp + From<u64> {
    // prime order of this field
    const MODULUS: Self;

    // mongomery reduction inverse
    const INV: u64;

    fn is_zero(self) -> bool;

    /// To bit representation in Big-endian
    fn to_bits(self) -> Bits;

    fn to_nafs(self) -> Nafs;

    fn double(self) -> Self;

    fn square(self) -> Self;

    fn double_assign(&mut self);

    fn square_assign(&mut self);

    fn pow_of_2(by: u64) -> Self;

    fn from_bytes_wide(bytes: &[u8; 64]) -> Self;

    fn to_raw_bytes(&self) -> Vec<u8>;
}

pub trait FieldRepr: Debug + BitAnd + BitXor + Sized {
    const LIMBS_LENGTH: usize;

    // map from montgomery to normal form
    fn montgomery_reduce(self) -> Self;
}
