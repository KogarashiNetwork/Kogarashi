use super::expression::SparseRow;
use super::matrix::Element;

use zkstd::common::PrimeField;

/// An rank-1 constraint of the form a * b = c, where a, b, and c are linear combinations of wires.
#[derive(Clone, Debug)]
pub struct Constraint<F: PrimeField> {
    pub a: SparseRow<F>,
    pub b: SparseRow<F>,
    pub c: SparseRow<F>,
}

impl<F: PrimeField> Constraint<F> {
    pub fn evaluate(&self, instance: &[Element<F>], witness: &[Element<F>]) -> (F, F, F) {
        let a_value = self.a.evaluate(instance, witness);
        let b_value = self.b.evaluate(instance, witness);
        let c_value = self.c.evaluate(instance, witness);

        (a_value, b_value, c_value)
    }
}