// This trait resresents elliptic curve and its scalar field
use super::{
    algebra::Group,
    comp::{Basic, ParityCmp},
    field::PrimeField,
};

/// elliptic curve rational points group
/// rational points group behaves as abelian group
pub trait Curve: ParityCmp + Basic + Group {
    // range field of curve
    type Range: PrimeField;

    // a param
    const PARAM_A: Self::Range;
    // b param
    const PARAM_B: Self::Range;

    // check that point is on curve
    fn is_identity(self) -> bool;

    // doubling this point
    fn double(self) -> Self;

    // check that point is on curve
    fn is_on_curve(self) -> bool;
}

/// rational point affine representation
/// affine representation check that a point is infinite by the struct field
pub trait Affine: Curve + Into<Self::Projective> + From<Self::Projective> {
    // projective coordinate representation
    type Projective: Projective;

    // convert affine to projective representation
    fn to_projective(self) -> Self::Projective;

    // get x coordinate
    fn get_x(&self) -> Self::Range;

    // get y coordinate
    fn get_y(&self) -> Self::Range;
}

/// rational point projective representation
/// projective representation check that a point is infinite by z coordinate
pub trait Projective: Curve + Into<Self::Affine> + From<Self::Affine> {
    // affine coordinate representation
    type Affine: Affine;

    // convert projective to affine representation
    fn to_affine(self) -> Self::Affine;

    // get x coordinate
    fn get_x(&self) -> Self::Range;

    // get y coordinate
    fn get_y(&self) -> Self::Range;

    // get z coordinate
    fn get_z(&self) -> Self::Range;

    // set x coordinate
    fn set_x(&mut self, value: Self::Range);

    // set y coordinate
    fn set_y(&mut self, value: Self::Range);

    // set z coordinate
    fn set_z(&mut self, value: Self::Range);
}
