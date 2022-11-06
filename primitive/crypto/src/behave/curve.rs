use super::{
    algebra::{Field, Ring},
    basic::Basic,
    parity::ParityCmp,
};

pub trait Curve: ParityCmp + Ring + Basic {}

pub trait PrimeField: ParityCmp + Field + Basic {
    const MODULUS: [u64; 4];

    const INV: u64;

    fn double(self) -> Self;

    fn square(self) -> Self;
}
