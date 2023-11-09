use super::field::FieldAssignment;
use crate::R1cs;

use zkstd::common::PrimeField;

pub struct PointAssignment<F: PrimeField> {
    x: FieldAssignment<F>,
    y: FieldAssignment<F>,
    z: FieldAssignment<F>,
}

impl<F: PrimeField> PointAssignment<F> {
    pub fn instance(cs: &mut R1cs<F>, x: F, y: F, is_infinity: bool) -> Self {
        let x = FieldAssignment::instance(cs, x);
        let y = FieldAssignment::instance(cs, y);
        let z = FieldAssignment::instance(cs, if is_infinity { F::zero() } else { F::one() });

        Self { x, y, z }
    }
}
