use crate::function::FunctionCircuit;

use core::marker::PhantomData;
use zkstd::circuit::prelude::{CircuitDriver, FieldAssignment};
use zkstd::matrix::DenseVectors;
use zkstd::r1cs::R1cs;

#[derive(Debug, Clone, Default)]
pub(crate) struct ExampleFunction<C: CircuitDriver> {
    mark: PhantomData<C>,
}

impl<C: CircuitDriver> FunctionCircuit<C> for ExampleFunction<C> {
    fn invoke(z: &DenseVectors<C::Scalar>) -> DenseVectors<C::Scalar> {
        let next_z = z[0] * z[0] * z[0] + z[0] + C::Scalar::from(5);
        DenseVectors::new(vec![next_z])
    }

    fn invoke_cs(
        cs: &mut R1cs<C>,
        z_i: Vec<FieldAssignment<C::Scalar>>,
    ) -> Vec<FieldAssignment<C::Scalar>> {
        let five = FieldAssignment::constant(&C::Scalar::from(5));
        let z_i_square = FieldAssignment::mul(cs, &z_i[0], &z_i[0]);
        let z_i_cube = FieldAssignment::mul(cs, &z_i_square, &z_i[0]);

        vec![&(&z_i_cube + &z_i[0]) + &five]
    }
}
