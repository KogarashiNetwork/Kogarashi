use super::matrix::{Element, SparseRow};

use zkstd::common::{PrimeField, Vec};

#[derive(Clone, Debug, Default)]
pub struct R1csStruct<F: PrimeField> {
    // matrix size
    m: usize,
    // public input size
    l: usize,
    pub(crate) constraints: Vec<Constraint<F>>,
}

impl<F: PrimeField> R1csStruct<F> {
    pub(crate) fn m(&self) -> usize {
        self.constraints.len()
    }
}

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
