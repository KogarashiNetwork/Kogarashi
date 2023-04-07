use crate::behave::{Affine, Curve, CurveExtend};

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
    // + Add<<Self as WeierstrassAffine>::Projective, Output = <Self as WeierstrassAffine>::Projective>
    // + Sub<<Self as WeierstrassAffine>::Projective, Output = <Self as WeierstrassAffine>::Projective>
    // + Add<Self, Output = <Self as WeierstrassAffine>::Projective>
    // + Sub<Self, Output = <Self as WeierstrassAffine>::Projective>
    // + Mul<<Self as CurveGroup>::Scalar, Output = <Self as WeierstrassAffine>::Projective>
    + Into<<Self as WeierstrassAffine>::Projective>
    + From<<Self as WeierstrassAffine>::Projective>
{
    // projective coordinate representation
    type Projective: Projective;

    // doubling this point
    fn double(self) -> <Self as WeierstrassAffine>::Projective; // Try to move it up

    // convert affine to projective representation
    fn to_projective(self) -> <Self as WeierstrassAffine>::Projective;
}

/// rational point projective representation
/// projective representation check that a point is infinite by z coordinate
pub trait Projective: CurveExtend {
    fn new(x: Self::Range, y: Self::Range, z: Self::Range) -> Self;

    // get z coordinate
    fn get_z(&self) -> Self::Range;
}
