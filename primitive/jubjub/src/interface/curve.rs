use crate::filed::Field;
use create::coordinate::Coordinate;

/// This expresses the curve equation
/// y^2 = x^3 + ax + b
pub trait Curve {
    type Field: Field;
    type Coordinate: Coordinate;

    fn a() -> Field;

    fn b() -> Field;

    fn is_on_curve() -> bool;

    fn generator() -> Coordinate;

    fn identity() -> Coordinate;
}
