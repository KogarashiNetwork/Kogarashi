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
}
