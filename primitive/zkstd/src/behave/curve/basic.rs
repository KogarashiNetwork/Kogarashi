use crate::{
    behave::{Basic, ParityCmp},
    common::CurveGroup,
};
use core::ops::{Add, AddAssign, MulAssign, Sub, SubAssign};

/// elliptic curve rational point affine representation
pub trait CurveAffine:
    CurveGroup + ParityCmp + Basic + From<<Self as CurveGroup>::Extended>
{
    fn to_extended(self) -> <Self as CurveGroup>::Extended;
}

/// extend curve point representation
/// projective, jacobian and so on
pub trait CurveExtended:
    CurveGroup
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
