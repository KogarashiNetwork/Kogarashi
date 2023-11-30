use crate::relaxed_r1cs::RelaxedR1csInstance;

use crate::circuit::MimcROCircuit;
use crate::hash::MIMC_ROUNDS;
use zkstd::circuit::prelude::{CircuitDriver, FieldAssignment, PointAssignment, R1cs};
use zkstd::common::CurveGroup;

pub(crate) struct RelaxedR1csInstanceAssignment<C: CircuitDriver> {
    pub(crate) commit_w: PointAssignment<C>,
    pub(crate) commit_e: PointAssignment<C>,
    pub(crate) u: FieldAssignment<C>,
    pub(crate) x: Vec<FieldAssignment<C>>,
}

impl<C: CircuitDriver> RelaxedR1csInstanceAssignment<C> {
    pub(crate) fn witness(
        cs: &mut R1cs<C>,
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
            commit_w.get_x().into(),
            commit_w.get_y().into(),
            commit_w.is_identity(),
        );
        let commit_e = PointAssignment::witness(
            cs,
            commit_e.get_x().into(),
            commit_e.get_y().into(),
            commit_e.is_identity(),
        );
        let u = FieldAssignment::witness(cs, *u);
        let x = x.iter().map(|x| FieldAssignment::witness(cs, x)).collect();

        Self {
            commit_w,
            commit_e,
            u,
            x,
        }
    }

    pub(crate) fn hash(
        &self,
        cs: &mut R1cs<C>,
        i: FieldAssignment<C>,
        z_0: Vec<FieldAssignment<C>>,
        z_i: Vec<FieldAssignment<C>>,
    ) -> FieldAssignment<C> {
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
    use bn_254::{Fr, G1Affine};
    use grumpkin::driver::GrumpkinDriver;
    use rand_core::OsRng;
    use zkstd::common::Group;
    use zkstd::matrix::DenseVectors;

    #[test]
    fn relaxed_instance_assignment() {
        let mut cs: R1cs<GrumpkinDriver> = R1cs::default();
        let instance = RelaxedR1csInstance {
            commit_e: G1Affine::random(OsRng),
            u: Fr::random(OsRng),
            commit_w: G1Affine::random(OsRng),
            x: DenseVectors::new(vec![Fr::random(OsRng); 1]),
        };

        let instance_assignment = RelaxedR1csInstanceAssignment::witness(&mut cs, &instance);
        FieldAssignment::eq_constant(&mut cs, &instance_assignment.u, &instance.u);
        // Think how to restrict
        FieldAssignment::eq_constant(&mut cs, &instance_assignment.x[0], &instance.x[0]);

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
