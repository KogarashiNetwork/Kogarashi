// This trait resresents elliptic curve and its scalar field
/// y^2 = x^3 + ax + b
use super::{algebra::Ring, basic::Basic, comp::ParityCmp, field::PrimeField};

pub trait Affine: ParityCmp + Basic + PartialEq + Eq {
    // scalar field of curve
    type ScalarField: PrimeField;

    // range field of curve
    type RangeField: PrimeField;

    // projective coordinate representation
    type Projective: Projective;

    // a param
    const PARAM_A: Self::RangeField;

    // b param
    const PARAM_B: Self::RangeField;

    // convert affine to projective representation
    fn to_projective(self) -> Self::Projective;

    // check that point is on curve
    fn is_identity(self) -> bool;

    // check that point is on curve
    fn is_on_curve(self) -> bool;
}

pub trait Projective: ParityCmp + Basic + Ring {
    // scalar field of curve
    type ScalarField: PrimeField;

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

    // check that point is on curve
    fn is_identity(self) -> bool;

    // doubling this point
    fn double(self) -> Self;

    // check that point is on curve
    fn is_on_curve(self) -> bool;

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
