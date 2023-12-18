use crate::relaxed_r1cs::RelaxedR1csInstance;

use crate::circuit::MimcROCircuit;
use crate::driver::scalar_as_base;
use crate::gadget::R1csInstanceAssignment;
use crate::hash::MIMC_ROUNDS;
use zkstd::circuit::prelude::{
    BinaryAssignment, CircuitDriver, FieldAssignment, PointAssignment, R1cs,
};
use zkstd::common::{CurveGroup, Ring};

#[derive(Clone)]
pub(crate) struct RelaxedR1csInstanceAssignment<C: CircuitDriver> {
    pub(crate) commit_w: PointAssignment<C::Base>,
    pub(crate) commit_e: PointAssignment<C::Base>,
    pub(crate) u: FieldAssignment<C::Base>,
    // TODO: change BigNatAssignment
    pub(crate) x0: FieldAssignment<C::Base>,
    pub(crate) x1: FieldAssignment<C::Base>,
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
        let x0 = FieldAssignment::witness(cs, scalar_as_base::<C>(x[0]));
        let x1 = FieldAssignment::witness(cs, scalar_as_base::<C>(x[1]));

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
            x0: instance.x0,
            x1: instance.x1,
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
        let x0 = FieldAssignment::conditional_select(cs, &a.x0, &b.x0, condition);
        let x1 = FieldAssignment::conditional_select(cs, &a.x1, &b.x1, condition);
        Self {
            commit_w,
            commit_e,
            u,
            x0,
            x1,
        }
    }

    pub(crate) fn absorb_by_transcript<const ROUNDS: usize>(
        &self,
        transcript: &mut MimcROCircuit<ROUNDS, C>,
    ) {
        transcript.append_point(self.commit_w.clone());
        transcript.append_point(self.commit_e.clone());
        transcript.append(self.u.clone());
        transcript.append(self.x0.clone());
        transcript.append(self.x1.clone());
    }

    pub(crate) fn hash<CS: CircuitDriver<Scalar = C::Base>>(
        &self,
        cs: &mut R1cs<CS>,
        i: FieldAssignment<C::Base>,
        z_0: Vec<FieldAssignment<C::Base>>,
        z_i: Vec<FieldAssignment<C::Base>>,
    ) -> FieldAssignment<C::Base> {
        let commit_e = self.commit_e.descale(cs);
        let commit_w = self.commit_w.descale(cs);
        MimcROCircuit::<MIMC_ROUNDS, C>::default().hash_vec(
            cs,
            vec![
                vec![i],
                z_0,
                z_i,
                vec![self.u.clone()],
                vec![self.x0.clone()],
                vec![self.x1.clone()],
                vec![commit_e.get_x(), commit_e.get_y(), commit_e.get_z()],
                vec![commit_w.get_x(), commit_w.get_y(), commit_w.get_z()],
            ]
            .concat(),
        )
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
        FieldAssignment::enforce_eq_constant(
            &mut cs,
            &instance_assignment.x0,
            &scalar_as_base::<GrumpkinDriver>(instance.x[0]),
        );
        FieldAssignment::enforce_eq_constant(
            &mut cs,
            &instance_assignment.x1,
            &scalar_as_base::<GrumpkinDriver>(instance.x[1]),
        );

        instance_assignment
            .commit_e
            .assert_equal_public_point(&mut cs, instance.commit_e.to_extended());
        instance_assignment
            .commit_w
            .assert_equal_public_point(&mut cs, instance.commit_w.to_extended());

        assert!(cs.is_sat());
    }
}
