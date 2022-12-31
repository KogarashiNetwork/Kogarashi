// This trait resresents elliptic curve and its scalar field
use super::{
    algebra::Group,
    comp::{Basic, ParityCmp},
    field::PrimeField,
};

/// elliptic curve rational points group
/// rational points group behaves as abelian group
pub trait Curve: ParityCmp + Basic + Group {
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
    // range field of curve
    type Range: PrimeField;
    // projective coordinate representation
    type Projective: Projective;

    // a param
    const PARAM_A: Self::Range;
    // b param
    const PARAM_B: Self::Range;

    // convert affine to projective representation
    fn to_projective(self) -> Self::Projective;
}

/// rational point projective representation
/// projective representation check that a point is infinite by z coordinate
pub trait Projective: Curve + Into<Self::Affine> + From<Self::Affine> {
    // range field of curve
    type RangeField: PrimeField;
    // affine coordinate representation
    type Affine: Affine;

    // a param
    const PARAM_A: Self::RangeField;
    // b param
    const PARAM_B: Self::RangeField;

    // convert projective to affine representation
    fn to_affine(self) -> Self::Affine;

    // get x coordinate
    fn get_x(&self) -> Self::RangeField;

    // get y coordinate
    fn get_y(&self) -> Self::RangeField;

    // get z coordinate
    fn get_z(&self) -> Self::RangeField;

    // set x coordinate
    fn set_x(&mut self, value: Self::RangeField);

    // set y coordinate
    fn set_y(&mut self, value: Self::RangeField);

    // set z coordinate
    fn set_z(&mut self, value: Self::RangeField);
}
