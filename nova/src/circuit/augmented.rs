use crate::function::FunctionCircuit;
use crate::relaxed_r1cs::RelaxedR1csInstance;
use zkstd::circuit::CircuitDriver;
use zkstd::matrix::DenseVectors;
use zkstd::r1cs::R1cs;

#[derive(Debug, Clone, Default)]
pub struct AugmentedFCircuit<C: CircuitDriver, FC: FunctionCircuit<C>> {
    pub i: usize,
    pub z_0: DenseVectors<C::Scalar>,
    pub z_i: DenseVectors<C::Scalar>,
    pub u_i: RelaxedR1csInstance<C>,
    pub U_i: RelaxedR1csInstance<C>,
    pub U_i1: RelaxedR1csInstance<C>,
    pub commit_t: C::Affine,
    pub f: FC,
    pub x: C::Scalar,
}

impl<C: CircuitDriver, FC: FunctionCircuit<C>> AugmentedFCircuit<C, FC> {
    fn generate(&self, cs: &mut R1cs<C>) {}
}
