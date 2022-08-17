use crate::fr::Fr;

/// Elliptic curve affine point
pub trait Coordinate {
    fn identity() -> Self;

    fn is_identity(&self) -> bool;

    fn constant_b() -> Fr;

    fn is_on_curve(&self) -> bool;
}
