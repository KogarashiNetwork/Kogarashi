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
pub struct AugmentedFCircuit<C: CircuitDriver, FC: FunctionCircuit<C>> {
    pub i: usize,
    pub z_0: DenseVectors<C::Scalar>,
    pub z_i: DenseVectors<C::Scalar>,
    pub u_single: RelaxedR1csInstance<C>,
    pub u_range: RelaxedR1csInstance<C>,
    pub u_range_next: RelaxedR1csInstance<C>,
    pub commit_t: C::Affine,
    pub f: PhantomData<FC>,
    pub x: C::Scalar,
}

impl<C: CircuitDriver, FC: FunctionCircuit<C>> Default for AugmentedFCircuit<C, FC> {
    fn default() -> Self {
        Self {
            i: 0,
            z_0: DenseVectors::zero(1),
            z_i: DenseVectors::zero(1),
            u_single: RelaxedR1csInstance::dummy(1),
            u_range: RelaxedR1csInstance::dummy(1),
            u_range_next: RelaxedR1csInstance::dummy(1),
            commit_t: C::Affine::ADDITIVE_IDENTITY,
            f: Default::default(),
            x: RelaxedR1csInstance::<C>::dummy(1).hash(
                1,
                &DenseVectors::zero(1),
                &FC::invoke(&DenseVectors::zero(1)),
            ),
        }
    }
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
        dbg!(cs.m_l_1());
        let z_i = self
            .z_i
            .iter()
            .map(|x| FieldAssignment::witness(cs, x))
            .collect::<Vec<_>>();
        dbg!(cs.m_l_1());

        let u_dummy_native = RelaxedR1csInstance::dummy(1);
        let u_dummy = RelaxedR1csInstanceAssignment::witness(cs, &u_dummy_native);
        dbg!(cs.m_l_1());
        let u_i = RelaxedR1csInstanceAssignment::witness(cs, &self.u_single);
        dbg!(cs.m_l_1());
        let u_range = RelaxedR1csInstanceAssignment::witness(cs, &self.u_range);
        dbg!(cs.m_l_1());
        let u_range_next = RelaxedR1csInstanceAssignment::witness(cs, &self.u_range_next);
        dbg!(cs.m_l_1());
        let commit_t = PointAssignment::witness(
            cs,
            self.commit_t.get_x().into(),
            self.commit_t.get_y().into(),
            self.commit_t.is_identity(),
        );
        dbg!(cs.m_l_1());
        let x = FieldAssignment::instance(cs, self.x);
        dbg!(cs.m_l_1());

        let z_next = FC::invoke_cs(cs, z_i.clone());
        dbg!(cs.m_l_1());
        let zero = FieldAssignment::constant(&C::Scalar::zero());
        dbg!(cs.m_l_1());
        let bin_true = BinaryAssignment::witness(cs, 1);
        dbg!(cs.m_l_1());

        let base_case = FieldAssignment::is_eq(cs, &i, &zero);
        dbg!(cs.m_l_1());
        let not_base_case = FieldAssignment::is_neq(cs, &i, &zero);
        dbg!(cs.m_l_1());
        dbg!(cs.m());
        dbg!(cs.m_l_1());

        // (1) check that ui.x = hash(vk, i, z0, zi, Ui), where ui.x is the public IO of ui
        let u_i_x = u_range.hash(cs, i.clone(), z_0.clone(), z_i);

        let check = FieldAssignment::is_eq(cs, &u_i.x[0], &u_i_x);
        BinaryAssignment::conditional_enforce_equal(cs, &check, &bin_true, &not_base_case);

        dbg!(cs.m());
        dbg!(cs.m_l_1());

        // (2) check that (ui.E, ui.u) = (u⊥.E, 1),
        let check = FieldAssignment::is_eq(cs, &u_i.commit_e.get_x(), &u_dummy.commit_e.get_x());
        BinaryAssignment::conditional_enforce_equal(cs, &check, &bin_true, &not_base_case);
        let check = FieldAssignment::is_eq(cs, &u_i.commit_e.get_y(), &u_dummy.commit_e.get_y());
        BinaryAssignment::conditional_enforce_equal(cs, &check, &bin_true, &not_base_case);
        let check = FieldAssignment::is_eq(cs, &u_i.commit_e.get_z(), &u_dummy.commit_e.get_z());
        BinaryAssignment::conditional_enforce_equal(cs, &check, &bin_true, &not_base_case);
        let check =
            FieldAssignment::is_eq(cs, &u_i.u, &FieldAssignment::constant(&C::Scalar::one()));
        BinaryAssignment::conditional_enforce_equal(cs, &check, &bin_true, &not_base_case);

        dbg!(cs.m());
        dbg!(cs.m_l_1());

        // (3) compute Ui+1 ← NIFS.V(vk, U, u, T )
        let r = Self::get_challenge(cs, &u_range, commit_t);
        let nifs_check = NifsCircuit::verify(cs, r, u_i, u_range.clone(), u_range_next.clone());
        BinaryAssignment::conditional_enforce_equal(cs, &nifs_check, &bin_true, &not_base_case);

        dbg!(cs.m());
        dbg!(cs.m_l_1());

        // 4. (base case) u_{i+1}.X == H(1, z_0, F(z_0)=F(z_i)=z_i1, U_i) (with U_i being dummy)
        let u_next_x_basecase = u_range.hash(
            cs,
            FieldAssignment::constant(&C::Scalar::one()),
            z_0.clone(),
            z_next.clone(),
        );

        // 4. (non-base case). u_{i+1}.x = H(i+1, z_0, z_i+1, U_{i+1})
        let u_next_x = u_range_next.hash(
            cs,
            &i + &FieldAssignment::constant(&C::Scalar::one()),
            z_0,
            z_next,
        );

        dbg!(cs.m());
        dbg!(cs.m_l_1());

        // constrain u_{i+1}.x for base case
        let check = FieldAssignment::is_eq(cs, &u_next_x_basecase, &x);
        BinaryAssignment::conditional_enforce_equal(cs, &check, &bin_true, &base_case);
        // constrain u_{i+1}.x for non base case
        let check = FieldAssignment::is_eq(cs, &u_next_x, &x);
        BinaryAssignment::conditional_enforce_equal(cs, &check, &bin_true, &not_base_case);

        dbg!(cs.m());
        dbg!(cs.m_l_1());
    }

    pub(crate) fn get_challenge(
        cs: &mut R1cs<C>,
        u_range: &RelaxedR1csInstanceAssignment<C>,
        commit_t: PointAssignment<C>,
    ) -> FieldAssignment<C> {
        let mut transcript = MimcROCircuit::<MIMC_ROUNDS, C>::default();
        transcript.append_point(commit_t);
        u_range.absorb_by_transcript(&mut transcript);
        transcript.squeeze(cs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::relaxed_r1cs::RelaxedR1csWitness;
    use crate::test::ExampleFunction;
    use crate::RelaxedR1cs;
    use grumpkin::driver::GrumpkinDriver;

    #[test]
    fn augmented_circuit_dummies() {
        let mut cs = R1cs::<GrumpkinDriver>::default();
        let augmented_circuit =
            AugmentedFCircuit::<GrumpkinDriver, ExampleFunction<GrumpkinDriver>>::default();
        augmented_circuit.generate(&mut cs);

        assert!(cs.is_sat());

        assert_eq!(cs.l(), 2);

        let u_dummy = RelaxedR1csInstance::dummy(cs.l() - 1);
        let w_dummy = RelaxedR1csWitness::dummy(cs.m_l_1(), cs.m());

        let mut running_r1cs = RelaxedR1cs::new(cs.clone());
        running_r1cs = running_r1cs.update(&u_dummy, &w_dummy);
        assert!(running_r1cs.is_sat());
    }
}
