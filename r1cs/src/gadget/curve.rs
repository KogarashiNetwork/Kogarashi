use super::field::FieldAssignment;
use crate::circuit::CircuitDriver;
use crate::R1cs;

use zkstd::common::{IntGroup, Ring};

pub struct PointAssignment<C: CircuitDriver> {
    x: FieldAssignment<C>,
    y: FieldAssignment<C>,
    z: FieldAssignment<C>,
}

impl<C: CircuitDriver> PointAssignment<C> {
    pub fn instance(cs: &mut R1cs<C>, x: C::Base, y: C::Base, is_infinity: bool) -> Self {
        let x = FieldAssignment::instance(cs, x);
        let y = FieldAssignment::instance(cs, y);
        let z = FieldAssignment::instance(
            cs,
            if is_infinity {
                C::Base::zero()
            } else {
                C::Base::one()
            },
        );

        Self { x, y, z }
    }

    pub fn double(&self, cs: &mut R1cs<C>) -> Self {
        let b3 = FieldAssignment::constant(&C::b3());
        let t0 = FieldAssignment::mul(cs, &self.y, &self.y);
        let z3 = FieldAssignment::add(cs, &t0, &t0);
        let z3 = FieldAssignment::add(cs, &z3, &z3);
        let z3 = FieldAssignment::add(cs, &z3, &z3);
        let t1 = FieldAssignment::mul(cs, &self.y, &self.z);
        let t2 = FieldAssignment::mul(cs, &self.z, &self.z);
        let t2 = FieldAssignment::mul(cs, &t2, &b3);
        let x3 = FieldAssignment::mul(cs, &t2, &z3);
        let y3 = FieldAssignment::add(cs, &t0, &t2);
        let z3 = FieldAssignment::mul(cs, &t1, &z3);
        let t1 = FieldAssignment::add(cs, &t2, &t2);
        let t2 = FieldAssignment::add(cs, &t1, &t2);
        let t0 = FieldAssignment::sub(cs, &t0, &t2);
        let y3 = FieldAssignment::mul(cs, &t0, &y3);
        let y3 = FieldAssignment::add(cs, &x3, &y3);
        let t1 = FieldAssignment::mul(cs, &self.x, &self.y);
        let x3 = FieldAssignment::mul(cs, &t0, &t1);
        let x3 = FieldAssignment::add(cs, &x3, &x3);

        Self {
            x: x3,
            y: y3,
            z: z3,
        }
    }
}
