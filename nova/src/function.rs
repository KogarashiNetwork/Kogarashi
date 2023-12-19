use std::fmt::Debug;
use zkstd::circuit::prelude::{CircuitDriver, FieldAssignment};
use zkstd::common::PrimeField;
use zkstd::matrix::DenseVectors;
use zkstd::r1cs::R1cs;

pub trait FunctionCircuit<F: PrimeField>: Clone + Debug + Default {
    fn invoke(z_i: &DenseVectors<F>) -> DenseVectors<F>;

    fn invoke_cs<CS: CircuitDriver<Scalar = F>>(
        cs: &mut R1cs<CS>,
        z_i: Vec<FieldAssignment<F>>,
    ) -> Vec<FieldAssignment<F>>;
}
