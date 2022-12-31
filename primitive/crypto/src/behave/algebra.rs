// trait resresenting abstract algebra concept
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// group trait which supports additive and scalar arithmetic
/// additive and scalar arithmetic hold associative and distributive property
/// any element has its inverse and these is the identity in group
/// existence of inverse is ensured for only additive arithmetic
pub trait Group:
    PartialEq
    + Eq
    + Add<Output = Self>
    + AddAssign
    + Neg<Output = Self>
    + Sub<Output = Self>
    + SubAssign
    + Mul<Self::Scalar, Output = Self>
    + MulAssign<Self::Scalar>
    + Sized
{
    // scalar domain
    type Scalar: Group;

    // generator of group
    const GENERATOR: Self;
    // additive identity of group
    // a * e = a for any a
    const ADDITIVE_IDENTITY: Self;

    // get inverse of group element
    fn invert(self) -> Option<Self>
    where
        Self: Sized;
}

/// ring trait which supports additive and multiplicative arithmetics
/// both arithmetics hold associative and distributive property
pub trait Ring: Group + Mul<Output = Self> + MulAssign + PartialOrd + Ord {
    const MULTIPLICATIVE_IDENTITY: Self;
}

/// field trait which ensures the existence of inverse for both multiplicative and additive arithmetic
/// hence field supports division for any element
pub trait Field: Ring + Div<Output = Self> + DivAssign {}
