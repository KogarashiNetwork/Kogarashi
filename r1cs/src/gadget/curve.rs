use super::field::FieldAssignment;

use zkstd::common::PrimeField;

pub struct PointAssignment<F: PrimeField> {
    x: FieldAssignment<F>,
    y: FieldAssignment<F>,
}

impl<F: PrimeField> PointAssignment<F> {
    pub fn new(x: FieldAssignment<F>, y: FieldAssignment<F>) -> Self {
        Self { x, y }
    }
}
