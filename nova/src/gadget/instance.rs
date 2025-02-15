use crate::relaxed_r1cs::R1csInstance;

use crate::driver::scalar_as_base;

use zkstd::circuit::prelude::{
    BinaryAssignment, CircuitDriver, FieldAssignment, PointAssignment, R1cs,
};
use zkstd::common::CurveGroup;

#[derive(Clone)]
pub(crate) struct R1csInstanceAssignment<C: CircuitDriver> {
    pub(crate) commit_w: PointAssignment<C::Base>,
    pub(crate) x0: FieldAssignment<C::Base>,
    pub(crate) x1: FieldAssignment<C::Base>,
}

impl<C: CircuitDriver> R1csInstanceAssignment<C> {
    pub(crate) fn witness<CS: CircuitDriver<Scalar = C::Base>>(
        cs: &mut R1cs<CS>,
        r1cs_instance: &R1csInstance<C>,
    ) -> Self {
        let R1csInstance { commit_w, x } = r1cs_instance;

        let commit_w = PointAssignment::witness(
            cs,
            commit_w.get_x(),
            commit_w.get_y(),
            commit_w.is_identity(),
        );

        let x0 = FieldAssignment::witness(cs, scalar_as_base::<C>(x[0]));
        let x1 = FieldAssignment::witness(cs, scalar_as_base::<C>(x[1]));

        Self { commit_w, x0, x1 }
    }

    pub fn conditional_select<CS: CircuitDriver<Scalar = C::Base>>(
        cs: &mut R1cs<CS>,
        a: &Self,
        b: &Self,
        condition: &BinaryAssignment,
    ) -> Self {
        let commit_w = PointAssignment::conditional_select(cs, &a.commit_w, &b.commit_w, condition);

        let x0 = FieldAssignment::conditional_select(cs, &a.x0, &b.x0, condition);
        let x1 = FieldAssignment::conditional_select(cs, &a.x1, &b.x1, condition);
        Self { commit_w, x0, x1 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::driver::{Bn254Driver, GrumpkinDriver};
    use bn_254::Fq;
    use grumpkin::Affine;
    use rand_core::OsRng;
    use zkstd::common::{BNAffine, Group};
    use zkstd::matrix::DenseVectors;

    #[test]
    fn instance_assignment() {
        let mut rng = OsRng;
        let mut cs: R1cs<Bn254Driver> = R1cs::default();
        let instance = R1csInstance::<GrumpkinDriver> {
            commit_w: Affine::random(&mut rng),
            x: DenseVectors::new(vec![Fq::random(&mut rng); 2]),
        };

        let instance_assignment = R1csInstanceAssignment::witness(&mut cs, &instance);
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
            .commit_w
            .assert_equal_public_point(&mut cs, instance.commit_w.to_extended());

        assert!(cs.is_sat());
    }
}
