// This trait resresents elliptic curve and its scalar field
use super::{
    algebra::Group,
    comp::{Basic, ParityCmp},
    field::PrimeField,
};
use core::ops::{Add, AddAssign, Mul, Sub, SubAssign};

/// elliptic curve rational points group
/// rational points group behaves as abelian group
pub trait Curve: ParityCmp + Basic {
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

    // get x coordinate
    fn get_x(&self) -> Self::Range;

    // get y coordinate
    fn get_y(&self) -> Self::Range;

    // set x coordinate
    fn set_x(&mut self, value: Self::Range);

    // set y coordinate
    fn set_y(&mut self, value: Self::Range);
}

/// extend curve point representation
/// projective, jacobian and so on
pub trait CurveExtend:
    Curve
    + Group
    + AddAssign<Self::Affine>
    + Add<Self::Affine, Output = Self>
    + SubAssign<Self::Affine>
    + Sub<Self::Affine, Output = Self>
    + Into<Self::Affine>
    + From<Self::Affine>
{
    // affine coordinate representation
    type Affine: Affine;

    // convert projective to affine representation
    fn to_affine(self) -> Self::Affine;
}

/// rational point affine representation
/// affine representation check that a point is infinite by the struct field
pub trait Affine:
    Curve
    + Add<Self::CurveExtend, Output = Self::CurveExtend>
    + Sub<Self::CurveExtend, Output = Self::CurveExtend>
    + Add<Self, Output = Self::CurveExtend>
    + Sub<Self, Output = Self::CurveExtend>
    + Mul<Self::Scalar, Output = Self::CurveExtend>
    + Into<Self::CurveExtend>
    + From<Self::CurveExtend>
{
    // scalar field of affine
    type Scalar: PrimeField;
    // projective coordinate representation
    type CurveExtend: CurveExtend;

    // convert affine to projective representation
    fn to_extend(self) -> Self::CurveExtend;
}

/// rational point projective representation
/// projective representation check that a point is infinite by z coordinate
pub trait Projective: CurveExtend {
    // get z coordinate
    fn get_z(&self) -> Self::Range;

    // set z coordinate
    fn set_z(&mut self, value: Self::Range);
}
