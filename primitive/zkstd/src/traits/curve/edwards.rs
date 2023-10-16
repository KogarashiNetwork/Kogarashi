use crate::traits::{CurveAffine, CurveExtended, CurveGroup};

pub trait TwistedEdwardsCurve: CurveGroup + Into<Self::Extended> + From<Self::Extended> {
    const PARAM_D: Self::Range;
}

pub trait TwistedEdwardsAffine: CurveAffine + TwistedEdwardsCurve {
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
