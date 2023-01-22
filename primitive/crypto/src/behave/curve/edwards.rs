use crate::behave::{Affine, Curve, CurveExtend};
use core::ops::{Add, Mul, Sub};

pub trait TwistedEdwardsCurve: Curve {
    // d param
    const PARAM_D: Self::Range;
}

pub trait TwistedEdwardsAffine:
    Affine
    + TwistedEdwardsCurve
    + Add<Self::CurveExtend, Output = Self::CurveExtend>
    + Sub<Self::CurveExtend, Output = Self::CurveExtend>
    + Add<Self, Output = Self::CurveExtend>
    + Sub<Self, Output = Self::CurveExtend>
    + Mul<Self::Scalar, Output = Self::CurveExtend>
    + Into<Self::CurveExtend>
    + From<Self::CurveExtend>
{
    type CurveExtend: CurveExtend;

    // doubling this point
    fn double(self) -> Self::CurveExtend;

    // convert affine to projective representation
    fn to_extend(self) -> Self::CurveExtend;
}

pub trait Extended: TwistedEdwardsCurve + CurveExtend {
    // twisted edwards curve d params
    const D: Self::Range;

    fn new(x: Self::Range, y: Self::Range, t: Self::Range, z: Self::Range) -> Self;

    // get t coordinate
    fn get_t(&self) -> Self::Range;

    // get z coordinate
    fn get_z(&self) -> Self::Range;
}
