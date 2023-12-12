use crate::relaxed_r1cs::RelaxedR1csInstance;

use crate::circuit::MimcROCircuit;
use crate::driver::scalar_as_base;
use crate::hash::MIMC_ROUNDS;
use zkstd::circuit::prelude::{CircuitDriver, FieldAssignment, PointAssignment, R1cs};
use zkstd::common::CurveGroup;

#[derive(Clone)]
pub(crate) struct RelaxedR1csInstanceAssignment<C: CircuitDriver> {
    pub(crate) commit_w: PointAssignment<C::Base>,
    pub(crate) commit_e: PointAssignment<C::Base>,
    pub(crate) u: FieldAssignment<C::Base>,
    pub(crate) x: Vec<FieldAssignment<C::Base>>,
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
        let x = x
            .iter()
            .map(|x| FieldAssignment::witness(cs, scalar_as_base::<C>(x)))
            .collect();

        Self {
            commit_w,
            commit_e,
            u,
            x,
        }
    }

    pub(crate) fn absorb_by_transcript<const ROUNDS: usize>(
        &self,
        transcript: &mut MimcROCircuit<ROUNDS, C>,
    ) {
        transcript.append_point(self.commit_w.clone());
        transcript.append_point(self.commit_e.clone());
        transcript.append(self.u.clone());
        for x in &self.x {
            transcript.append(x.clone());
        }
    }

    pub(crate) fn hash<CS: CircuitDriver<Scalar = C::Base>>(
        &self,
        cs: &mut R1cs<CS>,
        i: FieldAssignment<C::Base>,
        z_0: Vec<FieldAssignment<C::Base>>,
        z_i: Vec<FieldAssignment<C::Base>>,
    ) -> FieldAssignment<C::Base> {
        MimcROCircuit::<MIMC_ROUNDS, C>::default().hash_vec(
            cs,
            vec![
                vec![i],
                z_0,
                z_i,
                vec![self.u.clone()],
                self.x.clone(),
                vec![
                    self.commit_e.get_x(),
                    self.commit_e.get_y(),
                    self.commit_e.get_z(),
                ],
                vec![
                    self.commit_w.get_x(),
                    self.commit_w.get_y(),
                    self.commit_w.get_z(),
                ],
            ]
            .concat(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::driver::{Bn254Driver, GrumpkinDriver};
    use bn_254::Fq;
    use grumpkin::Affine;
    use rand_core::OsRng;
    use zkstd::common::Group;
    use zkstd::matrix::DenseVectors;

    // #[test]
    // fn instance_assignment_hash() {
    //     let mut cs: R1cs<Bn254Driver> = R1cs::default();
    //     let instance = RelaxedR1csInstance::<GrumpkinDriver> {
    //         commit_e: Affine::random(OsRng),
    //         u: Fq::random(OsRng),
    //         commit_w: Affine::random(OsRng),
    //         x: DenseVectors::new(vec![Fq::random(OsRng); 1]),
    //     };
    //
    //     let i = 3;
    //     let z_0 = DenseVectors::new(vec![Fr::from(3)]);
    //     let z_i = z_0.clone();
    //
    //     let hash = instance.hash::<Bn254Driver>(i, &z_0, &z_i);
    //
    //     let i_assignment = FieldAssignment::witness(&mut cs, Fr::from(i as u64));
    //     let z_0_assignment = z_0
    //         .iter()
    //         .map(|x| FieldAssignment::witness(&mut cs, x))
    //         .collect::<Vec<_>>();
    //     let z_i_assignment = z_i
    //         .iter()
    //         .map(|x| FieldAssignment::witness(&mut cs, x))
    //         .collect::<Vec<_>>();
    //     let instance_assignment = RelaxedR1csInstanceAssignment::witness(&mut cs, &instance);
    //
    //     let hash_circuit =
    //         instance_assignment.hash(&mut cs, i_assignment, z_0_assignment, z_i_assignment);
    //
    //     FieldAssignment::enforce_eq_constant(&mut cs, &hash_circuit, &hash);
    //     assert!(cs.is_sat());
    // }

    #[test]
    fn relaxed_instance_assignment() {
        let mut cs: R1cs<Bn254Driver> = R1cs::default();
        let instance = RelaxedR1csInstance::<GrumpkinDriver> {
            commit_e: Affine::random(OsRng),
            u: Fq::random(OsRng),
            commit_w: Affine::random(OsRng),
            x: DenseVectors::new(vec![Fq::random(OsRng); 1]),
        };

        let instance_assignment = RelaxedR1csInstanceAssignment::witness(&mut cs, &instance);
        FieldAssignment::enforce_eq_constant(
            &mut cs,
            &instance_assignment.u,
            &scalar_as_base::<GrumpkinDriver>(instance.u),
        );
        // TODO: Think how to restrict size to 1
        FieldAssignment::enforce_eq_constant(
            &mut cs,
            &instance_assignment.x[0],
            &scalar_as_base::<GrumpkinDriver>(instance.x[0]),
        );

        // Research about curve cycles

        // instance_assignment
        //     .commit_e
        //     .assert_equal_public_point(&mut cs, &instance.commit_e);
        // instance_assignment
        //     .commit_w
        //     .assert_equal_public_point(&mut cs, &instance.commit_w);

        assert!(cs.is_sat());
    }
}
