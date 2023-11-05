use crate::{
    common::{CurveGroup, Vec},
    traits::{ParallelCmp, ParityCmp},
};
use core::ops::{Add, AddAssign, MulAssign, Sub, SubAssign};

/// elliptic curve rational point affine representation
pub trait CurveAffine:
    CurveGroup<Affine = Self>
    + ParityCmp
    + ParallelCmp
    + From<Self::Extended>
    + Add<Self::Extended, Output = Self::Extended>
    + for<'a> Add<&'a Self::Extended, Output = Self::Extended>
    + for<'b> Add<&'b Self::Extended, Output = Self::Extended>
    + for<'a, 'b> Add<&'b Self::Extended, Output = Self::Extended>
    + Sub<Self::Extended, Output = Self::Extended>
    + for<'a> Sub<&'a Self::Extended, Output = Self::Extended>
    + for<'b> Sub<&'b Self::Extended, Output = Self::Extended>
    + for<'a, 'b> Sub<&'b Self::Extended, Output = Self::Extended>
{
    fn to_extended(self) -> Self::Extended;

    fn to_raw_bytes(self) -> Vec<u8>;
}

/// extend curve point representation
/// projective, jacobian and so on
pub trait CurveExtended:
    CurveGroup<Extended = Self>
    + ParallelCmp
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
    fn to_affine(self) -> Self::Affine;
}
