use super::field::PrimeField;

/// `Fr` and `Curve` necessary for pairing
pub trait Engine {
    type Fr: PrimeField;
}

/// Curve affine cordinate
pub trait CurveAffine {
    type Engine: Engine;

    fn zero() -> Self;

    fn one() -> Self;

    fn is_zero(&self) -> bool;
}
