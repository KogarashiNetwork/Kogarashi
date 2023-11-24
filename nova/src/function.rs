use r1cs::{prelude::CircuitDriver, DenseVectors};

pub trait Function<C: CircuitDriver> {
    fn invoke(z: &DenseVectors<C::Scalar>) -> DenseVectors<C::Scalar>;
}
