use zkstd::circuit::prelude::CircuitDriver;
use zkstd::matrix::DenseVectors;

pub trait Function<C: CircuitDriver> {
    fn invoke(z: &DenseVectors<C::Scalar>) -> DenseVectors<C::Scalar>;
}
