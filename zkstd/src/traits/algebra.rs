// trait resresenting abstract algebra concept
use crate::traits::{field::PrimeField, primitive::Basic};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use rand_core::RngCore;

/// any element has its inverse and these is the identity in group
/// existence of inverse is ensured for only additive arithmetic
pub trait Group: Basic + Eq + PartialEq + Send + Sync {
    // generator of group
    const ADDITIVE_GENERATOR: Self;

    // additive identity of group
    // a * e = a for any a
    const ADDITIVE_IDENTITY: Self;

    // return inverse of element
    fn invert(self) -> Option<Self>;

    // return random element
    fn random(rand: impl RngCore) -> Self;
}

/// integer group trait which supports additive arithmetic
/// additive arithmetic hold associative and distributive property
pub trait IntGroup:
    Group
    + Neg<Output = Self>
    + for<'a> Neg
    + Add<Output = Self>
    + for<'a> Add<&'a Self, Output = Self>
    + for<'b> Add<&'b Self, Output = Self>
    + for<'a, 'b> Add<&'b Self, Output = Self>
    + AddAssign
    + for<'b> AddAssign<&'b Self>
    + Sub<Output = Self>
    + for<'a> Sub<&'a Self, Output = Self>
    + for<'b> Sub<&'b Self, Output = Self>
    + for<'a, 'b> Sub<&'b Self, Output = Self>
    + SubAssign
    + for<'b> SubAssign<&'b Self>
{
    // return zero element
    fn zero() -> Self;
}

pub trait CurveGroup: Group {
    // range field of curve
    type Base: PrimeField;

    fn from_x_and_y(x: Self::Base, y: Self::Base) -> Self;

    // check that point is on curve
    fn is_identity(&self) -> bool;

    // check that point is on curve
    fn is_on_curve(self) -> bool;

    // get x coordinate
    fn get_x(&self) -> Self::Base;

    // get y coordinate
    fn get_y(&self) -> Self::Base;
}

/// ring trait which supports additive and multiplicative arithmetics
/// both arithmetics hold associative and distributive property
/// default element is multiplicative generator
pub trait Ring:
    IntGroup
    + PartialOrd
    + Ord
    + Default
    + Mul<Output = Self>
    + for<'a> Mul<&'a Self, Output = Self>
    + for<'b> Mul<&'b Self, Output = Self>
    + for<'a, 'b> Mul<&'b Self, Output = Self>
    + MulAssign
    + for<'b> MulAssign<&'b Self>
{
    const MULTIPLICATIVE_IDENTITY: Self;

    // return one element
    fn one() -> Self;
}

/// field trait which ensures the existence of inverse for both multiplicative and additive arithmetic
/// hence field supports division for any element
pub trait Field: Ring + Div<Output = Self> + DivAssign {}
