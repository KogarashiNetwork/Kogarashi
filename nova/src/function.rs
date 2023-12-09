use std::fmt::Debug;
use zkstd::circuit::prelude::{CircuitDriver, FieldAssignment};
use zkstd::matrix::DenseVectors;
use zkstd::r1cs::R1cs;

pub trait FunctionCircuit<C: CircuitDriver>: Clone + Debug + Default {
    fn invoke(z_i: &DenseVectors<C::Scalar>) -> DenseVectors<C::Scalar>;

    fn invoke_cs(
        cs: &mut R1cs<C>,
        z_i: Vec<FieldAssignment<C::Scalar>>,
    ) -> Vec<FieldAssignment<C::Scalar>>;
}
