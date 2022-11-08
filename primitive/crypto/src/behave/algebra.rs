// This trait resresents abstract algebra concept

use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// This is group trait
/// This has generator, identity and inverse
pub trait Group: PartialEq + Eq + PartialOrd + Ord {
    const MODULUS: Self;

    const GENERATOR: Self;

    const IDENTITY: Self;

    #[must_use]
    fn invert(self) -> Self;
}

/// This is ring trait
/// This has add, mul and sub operation
pub trait Ring: Group + Add + AddAssign + Mul + MulAssign + Neg + Sub + SubAssign + Sized {}

/// This is field trait
/// This has inverse for all element and operation
pub trait Field: Ring + Div + DivAssign {}
