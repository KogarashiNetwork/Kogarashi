use crate::function::FunctionCircuit;
use crate::gadget::RelaxedR1csInstanceAssignment;
use crate::relaxed_r1cs::RelaxedR1csInstance;
use std::marker::PhantomData;
use zkstd::circuit::prelude::{FieldAssignment, PointAssignment};
use zkstd::circuit::CircuitDriver;
use zkstd::common::{CurveGroup, IntGroup};
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
    pub f: PhantomData<FC>,
    pub x: C::Scalar,
}

impl<C: CircuitDriver, FC: FunctionCircuit<C>> AugmentedFCircuit<C, FC> {
    pub(crate) fn generate(&self, cs: &mut R1cs<C>) {
        // allocate inputs
        let i = FieldAssignment::witness(cs, C::Scalar::from(self.i as u64));
        let z_0 = self
            .z_0
            .iter()
            .map(|x| FieldAssignment::witness(cs, x))
            .collect::<Vec<_>>();
        let z_i = self
            .z_i
            .iter()
            .map(|x| FieldAssignment::witness(cs, x))
            .collect::<Vec<_>>();

        let u_def = RelaxedR1csInstanceAssignment::witness(cs, &RelaxedR1csInstance::default());
        let u_i = RelaxedR1csInstanceAssignment::witness(cs, &self.u_i);
        let U_i = RelaxedR1csInstanceAssignment::witness(cs, &self.U_i);
        let U_i1 = RelaxedR1csInstanceAssignment::witness(cs, &self.U_i1);
        let commit_t = PointAssignment::witness(
            cs,
            self.commit_t.get_x().into(),
            self.commit_t.get_y().into(),
            self.commit_t.is_identity(),
        );
        let x = FieldAssignment::witness(cs, self.x);

        let z_next = FC::invoke_cs(cs, z_i.clone());
        let zero = FieldAssignment::constant(&C::Scalar::zero());

        // realise `equal` with `BinaryAssignment` return type
        let base_case = FieldAssignment::is_eq(cs, &i, &zero);
        let not_base_case = FieldAssignment::is_neq(cs, &i, &zero);

        let u_i_x = U_i.hash(cs, i.clone(), z_0.clone(), z_i.clone());
    }
}
