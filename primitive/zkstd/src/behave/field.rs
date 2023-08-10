// This trait resresents prime field

use core::{
    fmt::Debug,
    ops::{Add, AddAssign, BitAnd, BitXor, Mul, MulAssign, Sub, SubAssign},
};

use super::{
    algebra::Field,
    comp::{Basic, ParityCmp},
};
use crate::arithmetic::utils::{Bits, Nafs};

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

/// This is prime field trait
pub trait PrimeField: Field + Basic + ParityCmp + From<u64> + RefOps {
    // prime order of this field
    const MODULUS: Self;

    // mongomery reduction inverse
    const INV: u64;

    fn is_zero(self) -> bool;

    fn to_bits(self) -> Bits;

    fn to_nafs(self) -> Nafs;

    fn double(self) -> Self;

    fn square(self) -> Self;

    fn double_assign(&mut self);

    fn square_assign(&mut self);
}

pub trait FieldRepr: Debug + BitAnd + BitXor + Sized {
    const LIMBS_LENGTH: usize;

    // map from montgomery to normal form
    fn montgomery_reduce(self) -> Self;
}
