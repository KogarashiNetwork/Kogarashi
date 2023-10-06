// trait resresenting abstract algebra concept
use crate::common::Basic;
use core::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};
use rand_core::RngCore;

use super::{CurveAffine, CurveExtended, FftField, PrimeField};

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
    + Copy
{
    // scalar domain
    type Scalar: PrimeField;

    // generator of group
    const ADDITIVE_GENERATOR: Self;
    // additive identity of group
    // a * e = a for any a
    const ADDITIVE_IDENTITY: Self;

    // return zero element
    fn zero() -> Self;

    // get inverse of group element
    fn invert(self) -> Option<Self>
    where
        Self: Sized;

    // get randome element
    fn random(rand: impl RngCore) -> Self;
}

pub trait CurveGroup:
    PartialEq
    + Eq
    + Sized
    + Copy
    + Debug
    + Default
    + Add<Self, Output = Self::Extended>
    + for<'a> Add<&'a Self, Output = Self::Extended>
    + for<'b> Add<&'b Self, Output = Self::Extended>
    + for<'a, 'b> Add<&'b Self, Output = Self::Extended>
    + Sub<Self, Output = Self::Extended>
    + for<'a> Sub<&'a Self, Output = Self::Extended>
    + for<'b> Sub<&'b Self, Output = Self::Extended>
    + for<'a, 'b> Sub<&'b Self, Output = Self::Extended>
    + Neg<Output = Self>
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

    // a param
    const PARAM_A: Self::Range;

    // generator of group
    const ADDITIVE_GENERATOR: Self;

    // additive identity of group
    // a * e = a for any a
    const ADDITIVE_IDENTITY: Self;

    // return zero element
    fn zero() -> Self;

    // check that point is on curve
    fn is_identity(&self) -> bool;

    // get inverse of group element
    fn invert(self) -> Option<Self>
    where
        Self: Sized;

    // get randome element
    fn random(rand: impl RngCore) -> Self::Extended;

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
pub trait Ring: Group + Mul<Output = Self> + MulAssign + PartialOrd + Ord + Default {
    const MULTIPLICATIVE_IDENTITY: Self;

    // return one element
    fn one() -> Self;
}

/// field trait which ensures the existence of inverse for both multiplicative and additive arithmetic
/// hence field supports division for any element
pub trait Field: Ring + Basic + Div<Output = Self> + DivAssign + 'static {}
