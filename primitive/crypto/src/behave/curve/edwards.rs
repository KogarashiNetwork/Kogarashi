extern crate alloc;
use alloc::boxed::Box;

use crate::behave::{Affine, Curve, CurveExtend};

pub trait TwistedEdwardsCurve: Curve {
    // d param
    const PARAM_D: Self::Range;
}

pub trait TwistedEdwardsAffine:
    Affine + TwistedEdwardsCurve + Into<Self::Extend> + From<Self::Extend>
{
    type Extend: CurveExtend;

    // doubling this point
    fn double(self) -> Self::Extend;

    // convert affine to projective representation
    fn to_extend(self) -> Self::Extend;

    fn from_raw_unchecked(x: Self::Range, y: Self::Range) -> Self;
}

pub trait Extended: TwistedEdwardsCurve + CurveExtend {
    fn new(x: Self::Range, y: Self::Range, t: Self::Range, z: Self::Range) -> Self;

    // get t coordinate
    fn get_t(&self) -> Self::Range;

    // get z coordinate
    fn get_z(&self) -> Self::Range;

    fn batch_normalize<'a>(y: &'a mut [Self]) -> Box<dyn Iterator<Item = Self::Affine> + 'a>;
}
