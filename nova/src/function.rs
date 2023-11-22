use r1cs::{CircuitDriver, DenseVectors};

pub trait Function<C: CircuitDriver> {
    fn invoke(z: &DenseVectors<C::Scalar>) -> DenseVectors<C::Scalar>;
}
