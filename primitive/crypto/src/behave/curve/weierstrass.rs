use crate::behave::{Affine, Curve, CurveExtend};
use core::ops::{Add, Mul, Sub};

/// elliptic curve rational points group
/// rational points group behaves as abelian group
pub trait WeierstrassCurve: Curve {
    // b param
    const PARAM_B: Self::Range;
}

/// rational point affine representation
/// affine representation check that a point is infinite by the struct field
pub trait WeierstrassAffine:
    Affine
    + WeierstrassCurve
    + Add<Self::Projective, Output = Self::Projective>
    + Sub<Self::Projective, Output = Self::Projective>
    + Add<Self, Output = Self::Projective>
    + Sub<Self, Output = Self::Projective>
    + Mul<Self::Scalar, Output = Self::Projective>
    + Into<Self::Projective>
    + From<Self::Projective>
{
    // projective coordinate representation
    type Projective: Projective;

    // doubling this point
    fn double(self) -> Self::Projective;

    // convert affine to projective representation
    fn to_projective(self) -> Self::Projective;
}

/// rational point projective representation
/// projective representation check that a point is infinite by z coordinate
pub trait Projective: CurveExtend {
    // get z coordinate
    fn get_z(&self) -> Self::Range;

    // set z coordinate
    fn set_z(&mut self, value: Self::Range);
}
