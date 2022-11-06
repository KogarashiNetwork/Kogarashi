// This trait resresents prime field

use super::{algebra::Field, basic::Basic, comp::ParityCmp};

/// This is prime field trait
pub trait PrimeField: ParityCmp + Field + Basic {
    // prime field order
    const MODULUS: [u64; 4];

    // mongomery reduction inverse
    const INV: u64;
}
