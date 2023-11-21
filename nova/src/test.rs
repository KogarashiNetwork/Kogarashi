use core::marker::PhantomData;

use crate::function::Function;

use r1cs::{CircuitDriver, DenseVectors};

pub(crate) struct ExampleFunction<C: CircuitDriver> {
    mark: PhantomData<C>,
}

impl<C: CircuitDriver> Function<C> for ExampleFunction<C> {
    fn invoke(z: &DenseVectors<C::Scalar>) -> DenseVectors<C::Scalar> {
        let next_z = z[0] * z[0] * z[0] + z[0] + C::Scalar::from(5);
        DenseVectors::new(vec![next_z])
    }
}
