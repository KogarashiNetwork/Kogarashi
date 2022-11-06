// This trait resresents elliptic curve and its scalar field

use super::{
    algebra::{Field, Ring},
    basic::Basic,
    parity::ParityCmp,
};

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

/// This is prime field trait
/// This is used for scalar field
pub trait PrimeField: ParityCmp + Field + Basic {
    // prime field order
    const MODULUS: [u64; 4];

    // mongomery reduction inverse
    const INV: u64;
}
