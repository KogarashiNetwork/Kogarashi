use crate::common::Vec;
use crate::traits::{CurveGroup, PrimeField};
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait TwistedEdwardsCurve: CurveGroup {
    // d param
    const PARAM_D: Self::Range;
    // scalar field of curve
    type Scalar: PrimeField + From<Self::Range>;
}

pub trait TwistedEdwardsAffine:
    TwistedEdwardsCurve
    + From<Self::Extended>
    + Neg<Output = Self>
    + for<'a> Neg<Output = Self>
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
    // extended coordinate representation
    type Extended: TwistedEdwardsExtended<Range = Self::Range>;

    fn from_raw_unchecked(x: Self::Range, y: Self::Range) -> Self;

    fn to_extended(self) -> Self::Extended;

    fn to_raw_bytes(self) -> Vec<u8>;

    // doubling this point
    fn double(self) -> Self::Extended;
}

pub trait TwistedEdwardsExtended:
    TwistedEdwardsCurve
    + Neg<Output = Self>
    + for<'a> Neg<Output = Self>
    + Add<Self, Output = Self>
    + for<'a> Add<&'a Self, Output = Self>
    + for<'b> Add<&'b Self, Output = Self>
    + for<'a, 'b> Add<&'b Self, Output = Self>
    + Add<Self::Affine, Output = Self>
    + for<'a> Add<&'a Self::Affine, Output = Self>
    + for<'b> Add<&'b Self::Affine, Output = Self>
    + for<'a, 'b> Add<&'b Self::Affine, Output = Self>
    + Sub<Self, Output = Self>
    + for<'a> Sub<&'a Self, Output = Self>
    + for<'b> Sub<&'b Self, Output = Self>
    + for<'a, 'b> Sub<&'b Self, Output = Self>
    + Mul<Self::Scalar, Output = Self>
    + for<'a> Mul<&'a Self::Scalar, Output = Self>
    + for<'b> Mul<&'b Self::Scalar, Output = Self>
    + for<'a, 'b> Mul<&'b Self::Scalar, Output = Self>
    + AddAssign<Self>
    + for<'a> AddAssign<&'a Self>
    + SubAssign<Self>
    + for<'a> SubAssign<&'a Self>
    + MulAssign<Self::Scalar>
    + for<'a> MulAssign<&'a Self::Scalar>
{
    // affine coordinate representation
    type Affine: TwistedEdwardsAffine<Range = Self::Range>;

    fn new(x: Self::Range, y: Self::Range, t: Self::Range, z: Self::Range) -> Self;

    // get t coordinate
    fn get_t(&self) -> Self::Range;

    // get z coordinate
    fn get_z(&self) -> Self::Range;

    // convert projective to affine representation
    fn to_affine(self) -> Self::Affine;

    // doubling this point
    fn double(self) -> Self;
}
