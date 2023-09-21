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
    // TODO: Integrate Extended and Projective
    type Projective: TwistedEdwardsExtended<Range = Self::Range>;
    fn new_projective(
        x: Self::Range,
        y: Self::Range,
        t: Self::Range,
        z: Self::Range,
    ) -> Self::Projective;
    fn from_raw_unchecked(x: Self::Range, y: Self::Range) -> Self;
    fn new_extended(
        x: Self::Range,
        y: Self::Range,
        t: Self::Range,
        z: Self::Range,
    ) -> Self::Extended;
}

pub trait TwistedEdwardsExtended: TwistedEdwardsCurve + CurveExtended {
    fn new(x: Self::Range, y: Self::Range, t: Self::Range, z: Self::Range) -> Self;

    // get t coordinate
    fn get_t(&self) -> Self::Range;
}
