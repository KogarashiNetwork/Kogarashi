use std::fmt::Debug;
use zkstd::circuit::prelude::{CircuitDriver, FieldAssignment};
use zkstd::common::{Decode, Encode, PrimeField};
use zkstd::matrix::DenseVectors;
use zkstd::r1cs::R1cs;

pub trait FunctionCircuit<F: PrimeField>:
    Clone + Debug + Default + PartialEq + Eq + Encode + Decode
{
    fn invoke(z_i: &DenseVectors<F>) -> DenseVectors<F>;

    fn invoke_cs<C: CircuitDriver<Scalar = F>>(
        cs: &mut R1cs<C>,
        z_i: Vec<FieldAssignment<F>>,
    ) -> Vec<FieldAssignment<F>>;
}
