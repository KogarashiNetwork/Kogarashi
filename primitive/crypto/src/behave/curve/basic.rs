use crate::{
    behave::{Basic, ParityCmp, PrimeField},
    common::CurveGroup,
};
use core::ops::{Add, AddAssign, Sub, SubAssign};

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
}

/// elliptic curve rational point affine representation
pub trait Affine: Curve {}

/// extend curve point representation
/// projective, jacobian and so on
pub trait CurveExtend:
    Curve
    // + RefOps
    + AddAssign<Self::Affine>
    + Add<Self::Affine, Output = Self>
    + SubAssign<Self::Affine>
    + Sub<Self::Affine, Output = Self>
    + Into<Self::Affine>
    + From<Self::Affine>
{
    // affine coordinate representation

    // doubling this point
    fn double(self) -> Self;

    // convert projective to affine representation
    fn to_affine(self) -> Self::Affine;
}
