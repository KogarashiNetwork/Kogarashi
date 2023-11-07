use crate::{
    common::{FftField, PrimeField, Vec},
    traits::{Group, ParallelCmp, ParityCmp},
};
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// elliptic curve rational points group
/// rational points group behaves as abelian group
pub trait WeierstrassCurve: Group {
    // b param
    const PARAM_B: Self::Range;
    // 3b param
    const PARAM_3B: Self::Range;

    // range field of curve
    type Range: PrimeField;

    // scalar field of curve
    type Scalar: FftField + From<Self::Range>;

    // check that point is on curve
    fn is_identity(&self) -> bool;

    // check that point is on curve
    fn is_on_curve(self) -> bool;

    // get x coordinate
    fn get_x(&self) -> Self::Range;

    // get y coordinate
    fn get_y(&self) -> Self::Range;
}

/// rational point affine representation
/// affine representation check that a point is infinite by the struct field
pub trait WeierstrassAffine:
    WeierstrassCurve
    + ParityCmp
    + ParallelCmp
    + From<Self::Extended>
    + Neg<Output = Self>
    + for<'a> Neg<Output = Self>
    + Add<Self, Output = Self::Extended>
    + for<'a> Add<&'a Self, Output = Self::Extended>
    + for<'b> Add<&'b Self, Output = Self::Extended>
    + for<'a, 'b> Add<&'b Self, Output = Self::Extended>
    + Add<Self::Extended, Output = Self::Extended>
    + for<'a> Add<&'a Self::Extended, Output = Self::Extended>
    + for<'b> Add<&'b Self::Extended, Output = Self::Extended>
    + for<'a, 'b> Add<&'b Self::Extended, Output = Self::Extended>
    + Sub<Self::Extended, Output = Self::Extended>
    + for<'a> Sub<&'a Self::Extended, Output = Self::Extended>
    + for<'b> Sub<&'b Self::Extended, Output = Self::Extended>
    + for<'a, 'b> Sub<&'b Self::Extended, Output = Self::Extended>
    + Mul<Self::Scalar, Output = Self::Extended>
    + for<'a> Mul<&'a Self::Scalar, Output = Self::Extended>
    + for<'b> Mul<&'b Self::Scalar, Output = Self::Extended>
    + for<'a, 'b> Mul<&'b Self::Scalar, Output = Self::Extended>
{
    // extented coordinate representation
    type Extended: WeierstrassProjective<Affine = Self, Range = Self::Range>;

    fn to_projective(self) -> Self::Extended;

    fn to_extended(self) -> Self::Extended;

    fn to_raw_bytes(self) -> Vec<u8>;

    // doubling this point
    fn double(self) -> Self::Extended;
}

/// rational point projective representation
/// projective representation check that a point is infinite by z coordinate
pub trait WeierstrassProjective:
    WeierstrassCurve
    + ParallelCmp
    + Into<Self::Affine>
    + From<Self::Affine>
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
    + AddAssign
    + AddAssign<Self::Affine>
    + for<'a> AddAssign<&'a Self::Affine>
    + Sub<Self::Affine, Output = Self>
    + for<'a> Sub<&'a Self::Affine, Output = Self>
    + for<'b> Sub<&'b Self::Affine, Output = Self>
    + for<'a, 'b> Sub<&'b Self::Affine, Output = Self>
    + SubAssign
    + SubAssign<Self::Affine>
    + for<'a> SubAssign<&'a Self::Affine>
    + Mul<Self::Scalar, Output = Self>
    + for<'a> Mul<&'a Self::Scalar, Output = Self>
    + for<'b> Mul<&'b Self::Scalar, Output = Self>
    + for<'a, 'b> Mul<&'b Self::Scalar, Output = Self>
    + MulAssign<Self::Scalar>
    + for<'a> MulAssign<&'a Self::Scalar>
{
    // affine coordinate representation
    type Affine: WeierstrassAffine<Range = Self::Range, Scalar = Self::Scalar, Extended = Self>;

    fn new(x: Self::Range, y: Self::Range, z: Self::Range) -> Self;

    // get z coordinate
    fn get_z(&self) -> Self::Range;

    // convert projective to affine representation
    fn to_affine(self) -> Self::Affine;

    // doubling this point
    fn double(self) -> Self;
}
