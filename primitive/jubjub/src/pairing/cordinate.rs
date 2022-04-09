use super::engine::Engine;
use super::field::PrimeField;

/// Elliptic curve affine point
pub trait CurveAffine {
    type Engine: Engine;
    type Scalar: PrimeField;

    fn zero() -> Self;

    fn one() -> Self;

    fn is_zero(&self) -> Self;
}
