// This trait resresents elliptic curve and its scalar field

use super::{algebra::Ring, basic::Basic, comp::ParityCmp, field::PrimeField};

/// This is curve trait
/// This has add and mul operation
/// y^2 = x^3 + ax + b
pub trait Curve: ParityCmp + Ring + Basic {
    // scalar field of curve
    type ScalarField: PrimeField;

    // a param
    fn const_a() -> Self::ScalarField;

    // b param
    fn const_b() -> Self::ScalarField;
}
