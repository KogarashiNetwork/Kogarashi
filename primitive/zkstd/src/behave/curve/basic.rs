use crate::{
    behave::{Basic, ParityCmp, PrimeField},
    common::CurveGroup,
};
use core::ops::{Add, AddAssign, MulAssign, Sub, SubAssign};

pub trait Curve: CurveGroup + ParityCmp + Basic {
    // range field of curve
    type Range: PrimeField;

    // a param
    const PARAM_A: Self::Range;

    // check that point is on curve
    fn is_on_curve(self) -> bool;

    // get x coordinate
    fn get_x(&self) -> Self::Range;

    // get y coordinate
    fn get_y(&self) -> Self::Range;

    // doubling this point
    fn double(self) -> <Self as CurveGroup>::Extended;
}

/// elliptic curve rational point affine representation
pub trait Affine: Curve + From<<Self as CurveGroup>::Extended> {
    fn to_extended(self) -> <Self as CurveGroup>::Extended;
}

/// extend curve point representation
/// projective, jacobian and so on
pub trait CurveExtended:
    Curve
    + AddAssign
    + AddAssign<Self::Affine>
    + for<'a> AddAssign<&'a Self::Affine>
    + Add<Self::Affine, Output = Self>
    + for<'a> Add<&'a Self::Affine, Output = Self>
    + for<'b> Add<&'b Self::Affine, Output = Self>
    + for<'a, 'b> Add<&'b Self::Affine, Output = Self>
    + SubAssign
    + SubAssign<Self::Affine>
    + for<'a> SubAssign<&'a Self::Affine>
    + Sub<Self::Affine, Output = Self>
    + for<'a> Sub<&'a Self::Affine, Output = Self>
    + for<'b> Sub<&'b Self::Affine, Output = Self>
    + for<'a, 'b> Sub<&'b Self::Affine, Output = Self>
    + MulAssign<Self::Scalar>
    + for<'a> MulAssign<&'a Self::Scalar>
    + Into<Self::Affine>
    + From<Self::Affine>
{
    // affine coordinate representation

    // get z coordinate
    fn get_z(&self) -> Self::Range;

    // convert projective to affine representation
    fn to_affine(self) -> <Self as CurveGroup>::Affine;
}
