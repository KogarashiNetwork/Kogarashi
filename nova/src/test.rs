use crate::function::FunctionCircuit;

use core::marker::PhantomData;
use zkstd::circuit::prelude::{CircuitDriver, FieldAssignment};
use zkstd::common::PrimeField;
use zkstd::matrix::DenseVectors;
use zkstd::r1cs::R1cs;

#[derive(Debug, Clone, Default)]
pub(crate) struct ExampleFunction<F: PrimeField> {
    mark: PhantomData<F>,
}

impl<F: PrimeField> FunctionCircuit<F> for ExampleFunction<F> {
    fn invoke(z: &DenseVectors<F>) -> DenseVectors<F> {
        let next_z = z[0] * z[0] * z[0] + z[0] + F::from(5);
        DenseVectors::new(vec![next_z])
    }

    fn invoke_cs<CS: CircuitDriver<Scalar = F>>(
        cs: &mut R1cs<CS>,
        z_i: Vec<FieldAssignment<F>>,
    ) -> Vec<FieldAssignment<F>> {
        let five = FieldAssignment::constant(&F::from(5));
        let z_i_square = FieldAssignment::mul(cs, &z_i[0], &z_i[0]);
        let z_i_cube = FieldAssignment::mul(cs, &z_i_square, &z_i[0]);

        vec![&(&z_i_cube + &z_i[0]) + &five]
    }
}
