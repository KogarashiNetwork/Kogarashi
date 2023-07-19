extern crate alloc;
use alloc::boxed::Box;

use crate::{
    behave::{Curve, CurveExtended},
    common::CurveGroup,
};

use super::Affine;

pub trait TwistedEdwardsCurve:
    Curve + Into<<Self as CurveGroup>::Extended> + From<<Self as CurveGroup>::Extended>
{
    const PARAM_D: Self::Range;
}

pub trait TwistedEdwardsAffine: TwistedEdwardsCurve + Affine {
    fn from_raw_unchecked(x: Self::Range, y: Self::Range) -> Self;
}

pub trait TwistedEdwardsExtended: TwistedEdwardsCurve + CurveExtended {
    fn new(x: Self::Range, y: Self::Range, t: Self::Range, z: Self::Range) -> Self;

    // get t coordinate
    fn get_t(&self) -> Self::Range;

    fn batch_normalize<'a>(y: &'a mut [Self]) -> Box<dyn Iterator<Item = Self::Affine> + 'a>;
}
