// This trait resresents elliptic curve and its scalar field

use super::{algebra::Ring, basic::Basic, comp::ParityCmp, field::PrimeField};

/// This is curve trait
/// This has add and mul operation
/// y^2 = x^3 + ax + b
pub trait Curve: ParityCmp + Ring + Basic {
    // projective coordinate representation
    type Projective: Projective;

    // affine coordinate representation
    type Affine: Affine;

    // scalar field of curve
    type ScalarField: PrimeField;

    // a param
    const PARAM_A: Self::ScalarField;

    // b param
    const PARAM_B: Self::ScalarField;

    // check that point is on curve
    fn is_on_curve(self, point: Self::Affine) -> bool;
}

pub trait Affine: ParityCmp + Basic {
    type Projective: Projective;

    // convert affine to projective representation
    fn to_projective(self) -> Self::Projective;
}

pub trait Projective: Ring + Basic {
    type Affine: Affine;

    // convert projective to affine representation
    fn to_affine(self) -> Self::Affine;

    // check that point is on curve
    fn is_identity(self) -> bool;
}
