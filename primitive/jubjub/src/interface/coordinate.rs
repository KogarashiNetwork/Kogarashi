/// Elliptic curve affine point
pub trait Coordinate {
    fn identity() -> Self;

    fn one() -> Self;

    fn is_identity(&self) -> bool;

    fn is_on_curve(&self) -> bool;
}
