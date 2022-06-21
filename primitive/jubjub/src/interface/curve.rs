use crate::coordinate::Affine;
use crate::fr::Fr;

/// This expresses the curve equation
/// y^2 = x^3 + ax + b
pub trait Curve {
    fn a() -> Fr;

    fn b() -> Fr;

    fn is_on_curve() -> bool;

    fn generator() -> Affine;

    fn identity() -> Affine;
}
