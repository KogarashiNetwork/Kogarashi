use crate::relaxed_r1cs::RelaxedR1csInstance;

use crate::circuit::MimcROCircuit;
use crate::driver::scalar_as_base;
use crate::gadget::big_nat::{BigNatAssignment, BN_LIMB_WIDTH, BN_N_LIMBS};
use crate::gadget::{f_to_nat, R1csInstanceAssignment};
use crate::hash::{HASH_BITS, MIMC_ROUNDS};
use zkstd::circuit::prelude::{
    BinaryAssignment, CircuitDriver, FieldAssignment, PointAssignment, R1cs,
};
use zkstd::common::{CurveGroup, Ring};

#[derive(Clone)]
pub(crate) struct RelaxedR1csInstanceAssignment<C: CircuitDriver> {
    pub(crate) commit_w: PointAssignment<C::Base>,
    pub(crate) commit_e: PointAssignment<C::Base>,
    pub(crate) u: FieldAssignment<C::Base>,
    pub(crate) x0: BigNatAssignment<C::Base>,
    pub(crate) x1: BigNatAssignment<C::Base>,
}

impl<C: CircuitDriver> RelaxedR1csInstanceAssignment<C> {
    pub(crate) fn witness<CS: CircuitDriver<Scalar = C::Base>>(
        cs: &mut R1cs<CS>,
        relaxed_r1cs_instance: &RelaxedR1csInstance<C>,
    ) -> Self {
        let RelaxedR1csInstance {
            commit_w,
            commit_e,
            u,
            x,
        } = relaxed_r1cs_instance;

        let commit_w = PointAssignment::witness(
            cs,
            commit_w.get_x(),
            commit_w.get_y(),
            commit_w.is_identity(),
        );
        let commit_e = PointAssignment::witness(
            cs,
            commit_e.get_x(),
            commit_e.get_y(),
            commit_e.is_identity(),
        );
        let u = FieldAssignment::witness(cs, scalar_as_base::<C>(*u));
        let x0 =
            BigNatAssignment::witness_from_big_int(cs, f_to_nat(&x[0]), BN_LIMB_WIDTH, BN_N_LIMBS);
        let x1 =
            BigNatAssignment::witness_from_big_int(cs, f_to_nat(&x[1]), BN_LIMB_WIDTH, BN_N_LIMBS);

        Self {
            commit_w,
            commit_e,
            u,
            x0,
            x1,
        }
    }

    /// E = 0, u = 1
    pub fn from_r1cs_instance<CS: CircuitDriver<Scalar = C::Base>>(
        cs: &mut R1cs<CS>,
        instance: R1csInstanceAssignment<C>,
    ) -> Self {
        let commit_e = PointAssignment::identity();
        Self {
            commit_w: instance.commit_w,
            commit_e,
            u: FieldAssignment::constant(&C::Base::one()),
            x0: BigNatAssignment::witness_from_field_assignment(
                cs,
                &instance.x0,
                BN_LIMB_WIDTH,
                BN_N_LIMBS,
            ),
            x1: BigNatAssignment::witness_from_field_assignment(
                cs,
                &instance.x1,
                BN_LIMB_WIDTH,
                BN_N_LIMBS,
            ),
        }
    }

    pub fn conditional_select<CS: CircuitDriver<Scalar = C::Base>>(
        cs: &mut R1cs<CS>,
        a: &Self,
        b: &Self,
        condition: &BinaryAssignment,
    ) -> Self {
        let commit_w = PointAssignment::conditional_select(cs, &a.commit_w, &b.commit_w, condition);
        let commit_e = PointAssignment::conditional_select(cs, &a.commit_e, &b.commit_e, condition);
        let u = FieldAssignment::conditional_select(cs, &a.u, &b.u, condition);
        let x0 = BigNatAssignment::conditional_select(cs, &a.x0, &b.x0, condition);
        let x1 = BigNatAssignment::conditional_select(cs, &a.x1, &b.x1, condition);
        Self {
            commit_w,
            commit_e,
            u,
            x0,
            x1,
        }
    }

    pub(crate) fn absorb_by_transcript<CS: CircuitDriver<Scalar = C::Base>, const ROUNDS: usize>(
        &self,
        cs: &mut R1cs<CS>,
        transcript: &mut MimcROCircuit<ROUNDS, C>,
    ) {
        let commit_e = self.commit_e.descale(cs);
        let commit_w = self.commit_w.descale(cs);
        transcript.append_point(commit_w);
        transcript.append_point(commit_e);
        transcript.append(self.u.clone());
        for limb in self.x0.as_limbs() {
            transcript.append(limb);
        }
        for limb in self.x1.as_limbs() {
            transcript.append(limb);
        }
    }

    pub(crate) fn hash<CS: CircuitDriver<Scalar = C::Base>>(
        &self,
        cs: &mut R1cs<CS>,
        i: FieldAssignment<C::Base>,
        z_0: Vec<FieldAssignment<C::Base>>,
        z_i: Vec<FieldAssignment<C::Base>>,
    ) -> FieldAssignment<C::Base> {
        let mut mimc_circuit = MimcROCircuit::<MIMC_ROUNDS, C>::default();
        mimc_circuit.append(i);
        mimc_circuit.append_vec(z_0);
        mimc_circuit.append_vec(z_i);
        self.absorb_by_transcript(cs, &mut mimc_circuit);
        mimc_circuit.squeeze(cs, HASH_BITS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::driver::{Bn254Driver, GrumpkinDriver};
    use bn_254::{Fq, Fr};
    use grumpkin::Affine;
    use rand_core::OsRng;
    use zkstd::common::{BNAffine, Group};
    use zkstd::matrix::DenseVectors;

    #[test]
    fn instance_assignment_hash() {
        let mut cs: R1cs<Bn254Driver> = R1cs::default();
        let instance = RelaxedR1csInstance::<GrumpkinDriver> {
            commit_e: Affine::random(OsRng),
            u: Fq::random(OsRng),
            commit_w: Affine::random(OsRng),
            x: DenseVectors::new(vec![Fq::random(OsRng); 2]),
        };

        let i = 3;
        let z_0 = DenseVectors::new(vec![Fr::from(3)]);
        let z_i = z_0.clone();

        let hash = instance.hash::<Bn254Driver>(i, &z_0, &z_i);

        let i_assignment = FieldAssignment::witness(&mut cs, Fr::from(i as u64));
        let z_0_assignment = z_0
            .iter()
            .map(|x| FieldAssignment::witness(&mut cs, x))
            .collect::<Vec<_>>();
        let z_i_assignment = z_i
            .iter()
            .map(|x| FieldAssignment::witness(&mut cs, x))
            .collect::<Vec<_>>();
        let instance_assignment = RelaxedR1csInstanceAssignment::witness(&mut cs, &instance);

        let hash_circuit =
            instance_assignment.hash(&mut cs, i_assignment, z_0_assignment, z_i_assignment); // E2::Base

        FieldAssignment::enforce_eq_constant(
            &mut cs,
            &hash_circuit,
            &scalar_as_base::<GrumpkinDriver>(hash),
        );
        assert!(cs.is_sat());
    }

    #[test]
    fn relaxed_instance_assignment() {
        let mut cs: R1cs<Bn254Driver> = R1cs::default();
        let instance = RelaxedR1csInstance::<GrumpkinDriver> {
            commit_e: Affine::random(OsRng),
            u: Fq::random(OsRng),
            commit_w: Affine::random(OsRng),
            x: DenseVectors::new(vec![Fq::random(OsRng); 2]),
        };

        let instance_assignment = RelaxedR1csInstanceAssignment::witness(&mut cs, &instance);
        FieldAssignment::enforce_eq_constant(
            &mut cs,
            &instance_assignment.u,
            &scalar_as_base::<GrumpkinDriver>(instance.u),
        );

        let x0_ass = BigNatAssignment::witness_from_big_int(
            &mut cs,
            f_to_nat(&instance.x[0]),
            BN_LIMB_WIDTH,
            BN_N_LIMBS,
        );
        let x1_ass = BigNatAssignment::witness_from_big_int(
            &mut cs,
            f_to_nat(&instance.x[0]),
            BN_LIMB_WIDTH,
            BN_N_LIMBS,
        );
        BigNatAssignment::enforce_eq(&mut cs, &instance_assignment.x0, &x0_ass);
        BigNatAssignment::enforce_eq(&mut cs, &instance_assignment.x1, &x1_ass);

        instance_assignment
            .commit_e
            .assert_equal_public_point(&mut cs, instance.commit_e.to_extended());
        instance_assignment
            .commit_w
            .assert_equal_public_point(&mut cs, instance.commit_w.to_extended());

        assert!(cs.is_sat());
    }
}
