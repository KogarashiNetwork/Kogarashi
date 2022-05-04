use crate::entity::Fr;
use crate::interface::Engine;

/// This is the pairing Engine
pub trait JubjubEngine: Engine {}

/// This is the affine point
pub struct Point {
    x: Fr,
    y: Fr,
    z: Fr,
}
