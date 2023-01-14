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
