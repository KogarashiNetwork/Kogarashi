// This trait resresents elliptic curve and its scalar field
/// y^2 = x^3 + ax + b
use super::{algebra::Ring, basic::Basic, comp::ParityCmp, field::PrimeField};

pub trait Curve {
    // scalar field of curve
    type ScalarField: PrimeField;

    // affine coordinate representation
    type Affine: Affine;

    // projective coordinate representation
    type Projective: Projective;

    // a param
    const PARAM_A: Self::ScalarField;

    // b param
    const PARAM_B: Self::ScalarField;
}

pub trait Affine: ParityCmp + Basic {
    // scalar field of curve
    type ScalarField: PrimeField;

    // projective coordinate representation
    type Projective: Projective;

    // a param
    const PARAM_A: Self::ScalarField;

    // b param
    const PARAM_B: Self::ScalarField;

    // convert affine to projective representation
    fn to_projective(self) -> Self::Projective;

    // check that point is on curve
    fn is_on_curve(self) -> bool;
}

pub trait Projective: ParityCmp + Basic + Ring {
    // scalar field of curve
    type ScalarField: PrimeField;

    // affine coordinate representation
    type Affine: Affine;

    // a param
    const PARAM_A: Self::ScalarField;

    // b param
    const PARAM_B: Self::ScalarField;

    // convert projective to affine representation
    fn to_affine(self) -> Self::Affine;

    // check that point is on curve
    fn is_identity(self) -> bool;
}
