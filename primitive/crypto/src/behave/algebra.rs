// This trait resresents abstract algebra concept

use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// This is group trait
/// This has generator, identity and inverse
pub trait Group: PartialEq + Eq {
    const GENERATOR: Self;
    const IDENTITY: Self;

    fn invert(self) -> Option<Self>
    where
        Self: Sized;
}

/// This is ring trait
/// This has add, mul and sub operation
pub trait Ring:
    Group + Add<Output = Self> + AddAssign + Neg<Output = Self> + Sub<Output = Self> + SubAssign + Sized
{
}

/// This is field trait
/// This has inverse for all element and operation
pub trait Field: Ring + Mul<Output = Self> + MulAssign + Div<Output = Self> + DivAssign {}
