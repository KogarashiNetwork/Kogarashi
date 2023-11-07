// trait resresenting abstract algebra concept
use crate::traits::primitive::Basic;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use rand_core::RngCore;

use super::{CurveAffine, CurveExtended, FftField, PrimeField};

/// any element has its inverse and these is the identity in group
/// existence of inverse is ensured for only additive arithmetic
pub trait Group: Basic + Eq + PartialEq {
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

pub trait CurveGroup:
    Group
    + Neg<Output = Self>
    + Add<Self, Output = Self::Extended>
    + for<'a> Add<&'a Self, Output = Self::Extended>
    + for<'b> Add<&'b Self, Output = Self::Extended>
    + for<'a, 'b> Add<&'b Self, Output = Self::Extended>
    + Sub<Self, Output = Self::Extended>
    + for<'a> Sub<&'a Self, Output = Self::Extended>
    + for<'b> Sub<&'b Self, Output = Self::Extended>
    + for<'a, 'b> Sub<&'b Self, Output = Self::Extended>
    + Mul<Self::Scalar, Output = Self::Extended>
    + for<'a> Mul<&'a Self::Scalar, Output = Self::Extended>
    + for<'b> Mul<&'b Self::Scalar, Output = Self::Extended>
    + for<'a, 'b> Mul<&'b Self::Scalar, Output = Self::Extended>
{
    // range field of curve
    type Range: PrimeField;

    // scalar field of curve
    type Scalar: FftField + From<Self::Range>;

    // curve group
    type Affine: CurveAffine<Range = Self::Range, Scalar = Self::Scalar, Extended = Self::Extended>;
    type Extended: CurveExtended<Range = Self::Range, Scalar = Self::Scalar, Affine = Self::Affine>;

    // check that point is on curve
    fn is_identity(&self) -> bool;

    // check that point is on curve
    fn is_on_curve(self) -> bool;

    // get x coordinate
    fn get_x(&self) -> Self::Range;

    // get y coordinate
    fn get_y(&self) -> Self::Range;

    // doubling this point
    fn double(self) -> <Self as CurveGroup>::Extended;
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
