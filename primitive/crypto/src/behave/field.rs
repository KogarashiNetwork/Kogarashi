// This trait resresents prime field

use super::{algebra::Field, basic::Basic, comp::ParityCmp};

/// This is prime field trait
pub trait PrimeField: Field + Basic + ParityCmp {
    const MODULUS: Self;

    // mongomery reduction inverse
    const INV: u64;

    #[must_use]
    fn double(self) -> Self;

    #[must_use]
    fn square(self) -> Self;

    #[must_use]
    fn double_assign(&mut self);

    #[must_use]
    fn square_assign(&mut self);
}
