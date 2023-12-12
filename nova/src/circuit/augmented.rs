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
    pub i: usize,
    pub z_0: DenseVectors<C::Base>,
    pub z_i: Option<DenseVectors<C::Base>>,
    pub u_single: Option<RelaxedR1csInstance<C>>,
    pub u_range: Option<RelaxedR1csInstance<C>>,
    pub u_range_next: Option<RelaxedR1csInstance<C>>, // Remove
    pub commit_t: Option<C::Affine>,
    pub f: PhantomData<FC>,
    pub x: C::Base, // Remove
}

impl<C: CircuitDriver, FC: FunctionCircuit<C::Base>> Default for AugmentedFCircuit<C, FC> {
    fn default() -> Self {
        Self {
            i: 0,
            z_0: DenseVectors::zero(1),
            z_i: Some(DenseVectors::zero(1)),
            u_single: Some(RelaxedR1csInstance::dummy(1)),
            u_range: Some(RelaxedR1csInstance::dummy(1)),
            u_range_next: Some(RelaxedR1csInstance::dummy(1)),
            commit_t: Some(C::Affine::ADDITIVE_IDENTITY),
            f: Default::default(),
            x: C::Base::zero(),
        }
    }
}

impl<C: CircuitDriver, FC: FunctionCircuit<C::Base>> AugmentedFCircuit<C, FC> {
    pub(crate) fn generate<CS: CircuitDriver<Scalar = C::Base>>(&self, cs: &mut R1cs<CS>) {
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

        let u_dummy_native = RelaxedR1csInstance::<C>::dummy(1);
        let u_dummy = RelaxedR1csInstanceAssignment::witness(cs, &u_dummy_native);
        let u_i = RelaxedR1csInstanceAssignment::witness(
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
        let u_range_next = RelaxedR1csInstanceAssignment::witness(
            cs,
            &self.u_range_next.clone().unwrap_or(u_dummy_native),
        );

        let commit_t = self.commit_t.unwrap_or(C::Affine::ADDITIVE_IDENTITY);
        let commit_t = PointAssignment::witness(
            cs,
            commit_t.get_x(),
            commit_t.get_y(),
            commit_t.is_identity(),
        );
        let x = FieldAssignment::instance(cs, self.x);

        let z_next = FC::invoke_cs(cs, z_i.clone());
        let zero = FieldAssignment::constant(&C::Base::zero());
        let bin_true = BinaryAssignment::witness(cs, 1);

        let base_case = FieldAssignment::is_eq(cs, &i, &zero);
        let not_base_case = FieldAssignment::is_neq(cs, &i, &zero);

        // (1) check that ui.x = hash(vk, i, z0, zi, Ui), where ui.x is the public IO of ui
        let u_i_x = u_range.hash(cs, i.clone(), z_0.clone(), z_i);
        FieldAssignment::conditional_enforce_equal(cs, &u_i.x[0], &u_i_x, &not_base_case);

        // (2) check that (ui.E, ui.u) = (u⊥.E, 1),
        FieldAssignment::conditional_enforce_equal(
            cs,
            &u_i.commit_e.get_x(),
            &u_dummy.commit_e.get_x(),
            &not_base_case,
        );
        FieldAssignment::conditional_enforce_equal(
            cs,
            &u_i.commit_e.get_y(),
            &u_dummy.commit_e.get_y(),
            &not_base_case,
        );
        FieldAssignment::conditional_enforce_equal(
            cs,
            &u_i.commit_e.get_z(),
            &u_dummy.commit_e.get_z(),
            &not_base_case,
        );
        FieldAssignment::conditional_enforce_equal(
            cs,
            &u_i.u,
            &FieldAssignment::constant(&C::Base::one()),
            &not_base_case,
        );

        // (3) Verify Ui+1 ← NIFS.V(vk, U, u, T )
        let r = Self::get_challenge(cs, &u_range, commit_t);
        let nifs_check = NifsCircuit::verify(cs, r, u_i, u_range.clone(), u_range_next.clone());
        BinaryAssignment::conditional_enforce_equal(cs, &nifs_check, &bin_true, &not_base_case);

        // 4. (base case) u_{i+1}.X == H(1, z_0, F(z_0)=F(z_i)=z_i1, U_i) (with U_i being dummy)
        let u_next_x_basecase = u_range.hash(
            cs,
            FieldAssignment::constant(&C::Base::one()),
            z_0.clone(),
            z_next.clone(),
        );

        // 4. (non-base case). u_{i+1}.x = H(i+1, z_0, z_i+1, U_{i+1})
        let u_next_x = u_range_next.hash(
            cs,
            &i + &FieldAssignment::constant(&C::Base::one()),
            z_0,
            z_next,
        );

        // constrain u_{i+1}.x for base case
        FieldAssignment::conditional_enforce_equal(cs, &u_next_x_basecase, &x, &base_case);
        // constrain u_{i+1}.x for non base case
        FieldAssignment::conditional_enforce_equal(cs, &u_next_x, &x, &not_base_case);
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

        assert!(cs.is_sat());

        assert_eq!(cs.l(), 2);

        let u_dummy = RelaxedR1csInstance::dummy(cs.l() - 1);
        let w_dummy = RelaxedR1csWitness::dummy(cs.m_l_1(), cs.m());

        let running_r1cs = R1csShape::from(cs);
        assert!(running_r1cs.is_sat(&u_dummy, &w_dummy));
    }
}
