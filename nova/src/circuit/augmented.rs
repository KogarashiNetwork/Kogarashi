use crate::circuit::nifs::NifsCircuit;
use crate::circuit::MimcROCircuit;
use crate::function::FunctionCircuit;
use crate::gadget::RelaxedR1csInstanceAssignment;
use crate::hash::MIMC_ROUNDS;
use crate::relaxed_r1cs::RelaxedR1csInstance;
use std::marker::PhantomData;
use zkstd::circuit::prelude::{BinaryAssignment, FieldAssignment, PointAssignment};
use zkstd::circuit::CircuitDriver;
use zkstd::common::{CurveGroup, Group, IntGroup, Ring};
use zkstd::matrix::DenseVectors;
use zkstd::r1cs::R1cs;

#[derive(Debug, Clone)]
pub struct AugmentedFCircuit<C: CircuitDriver, FC: FunctionCircuit<C::Base>> {
    pub is_primary: bool,
    pub i: usize,
    pub z_0: DenseVectors<C::Base>,
    pub z_i: Option<DenseVectors<C::Base>>,
    pub u_single: Option<RelaxedR1csInstance<C>>,
    pub u_range: Option<RelaxedR1csInstance<C>>,
    pub commit_t: Option<C::Affine>,
    pub f: PhantomData<FC>,
}

impl<C: CircuitDriver, FC: FunctionCircuit<C::Base>> Default for AugmentedFCircuit<C, FC> {
    fn default() -> Self {
        Self {
            is_primary: true,
            i: 0,
            z_0: DenseVectors::zero(1),
            z_i: Some(DenseVectors::zero(1)),
            u_single: Some(RelaxedR1csInstance::dummy(2)),
            u_range: Some(RelaxedR1csInstance::dummy(2)),
            commit_t: Some(C::Affine::ADDITIVE_IDENTITY),
            f: Default::default(),
        }
    }
}

impl<C: CircuitDriver, FC: FunctionCircuit<C::Base>> AugmentedFCircuit<C, FC> {
    pub(crate) fn generate<CS: CircuitDriver<Scalar = C::Base>>(
        &self,
        cs: &mut R1cs<CS>,
    ) -> Vec<FieldAssignment<C::Base>> {
        // allocate inputs
        let i = FieldAssignment::witness(cs, C::Base::from(self.i as u64));
        let z_0 = self
            .z_0
            .iter()
            .map(|x| FieldAssignment::witness(cs, x))
            .collect::<Vec<_>>();
        let z_i = self
            .z_i
            .clone()
            .unwrap_or_else(|| self.z_0.clone())
            .iter()
            .map(|x| FieldAssignment::witness(cs, x))
            .collect::<Vec<_>>();

        let u_dummy_native = RelaxedR1csInstance::<C>::dummy(2);
        let u_dummy = RelaxedR1csInstanceAssignment::witness(cs, &u_dummy_native);
        let u_single = RelaxedR1csInstanceAssignment::witness(
            cs,
            &self
                .u_single
                .clone()
                .unwrap_or_else(|| u_dummy_native.clone()),
        );
        let u_range = RelaxedR1csInstanceAssignment::witness(
            cs,
            &self
                .u_range
                .clone()
                .unwrap_or_else(|| u_dummy_native.clone()),
        );

        let commit_t = self.commit_t.unwrap_or(C::Affine::ADDITIVE_IDENTITY);
        let commit_t = PointAssignment::witness(
            cs,
            commit_t.get_x(),
            commit_t.get_y(),
            commit_t.is_identity(),
        );

        let zero = FieldAssignment::constant(&C::Base::zero());
        let bin_true = BinaryAssignment::witness(cs, 1);

        let base_case = FieldAssignment::is_eq(cs, &i, &zero);
        let not_base_case = FieldAssignment::is_neq(cs, &i, &zero);

        // base case
        let u_range_next_base = if self.is_primary {
            u_dummy
        } else {
            u_single.clone()
        };

        // (1) check that ui.x = hash(vk, i, z0, zi, Ui), where ui.x is the public IO of ui
        let u_i_x = u_range.hash(cs, i.clone(), z_0.clone(), z_i.clone());
        FieldAssignment::conditional_enforce_equal(cs, &u_single.x0, &u_i_x, &not_base_case);

        // // (2) check that (ui.E, ui.u) = (u⊥.E, 1),
        // FieldAssignment::conditional_enforce_equal(
        //     cs,
        //     &u_single.commit_e.get_x(),
        //     &u_dummy.commit_e.get_x(),
        //     &not_base_case,
        // );
        // FieldAssignment::conditional_enforce_equal(
        //     cs,
        //     &u_single.commit_e.get_y(),
        //     &u_dummy.commit_e.get_y(),
        //     &not_base_case,
        // );
        // FieldAssignment::conditional_enforce_equal(
        //     cs,
        //     &u_single.commit_e.get_z(),
        //     &u_dummy.commit_e.get_z(),
        //     &not_base_case,
        // );
        // FieldAssignment::conditional_enforce_equal(
        //     cs,
        //     &u_single.u,
        //     &FieldAssignment::constant(&C::Base::one()),
        //     &not_base_case,
        // );

        // (3) Generate Ui+1 ← NIFS.V(vk, U, u, T)
        let r = Self::get_challenge(cs, &u_range, commit_t.clone());
        let u_range_next_non_base =
            NifsCircuit::verify(cs, r, u_single.clone(), u_range.clone(), commit_t);

        let u_range_next = RelaxedR1csInstanceAssignment::conditional_select(
            cs,
            &u_range_next_base,
            &u_range_next_non_base,
            &base_case,
        );

        let z_next = FC::invoke_cs(cs, z_i);

        // println!(
        //     "Hash(\n{:?}\n{:?}\n{:?}\n)",
        //     (&i + &FieldAssignment::constant(&C::Base::one())).value(cs),
        //     z_0.iter().map(|x| x.value(cs)).collect::<Vec<_>>(),
        //     z_next.iter().map(|x| x.value(cs)).collect::<Vec<_>>()
        // );
        // println!(
        //     "U = (\n{:?}\n{:?}\n{:?}\n)",
        //     u_range_next.u.value(cs),
        //     u_range_next.x0.value(cs),
        //     u_range_next.x1.value(cs)
        // );
        let u_next_x = u_range_next.hash(
            cs,
            &i + &FieldAssignment::constant(&C::Base::one()),
            z_0,
            z_next.clone(),
        );

        let x0 = FieldAssignment::inputize(cs, u_single.x1);
        let x1 = FieldAssignment::inputize(cs, u_next_x);

        z_next
    }

    pub(crate) fn get_challenge<CS: CircuitDriver<Scalar = C::Base>>(
        cs: &mut R1cs<CS>,
        u_range: &RelaxedR1csInstanceAssignment<C>,
        commit_t: PointAssignment<C::Base>,
    ) -> FieldAssignment<C::Base> {
        let mut transcript = MimcROCircuit::<MIMC_ROUNDS, C>::default();
        transcript.append_point(commit_t);
        u_range.absorb_by_transcript(&mut transcript);
        transcript.squeeze(cs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::driver::{Bn254Driver, GrumpkinDriver};
    use crate::relaxed_r1cs::{R1csShape, RelaxedR1csWitness};
    use crate::test::ExampleFunction;
    use bn_254::Fr;

    #[test]
    fn augmented_circuit_dummies() {
        let mut cs = R1cs::<Bn254Driver>::default();
        let augmented_circuit = AugmentedFCircuit::<GrumpkinDriver, ExampleFunction<Fr>>::default();
        augmented_circuit.generate(&mut cs);
        let shape = R1csShape::from(cs);
        let u_dummy = RelaxedR1csInstance::dummy(shape.l());
        let w_dummy = RelaxedR1csWitness::dummy(shape.m_l_1(), shape.m());

        assert!(shape.is_sat(&u_dummy, &w_dummy));
    }
}
