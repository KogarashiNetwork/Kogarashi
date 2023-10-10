use crate::r1cs::expression::Expression;
use crate::r1cs::wire_values::WireValues;

use zkstd::common::Field;

/// An rank-1 constraint of the form a * b = c, where a, b, and c are linear combinations of wires.
#[derive(Clone, Debug)]
pub struct Constraint<F: Field> {
    pub a: Expression<F>,
    pub b: Expression<F>,
    pub c: Expression<F>,
}

#[allow(dead_code)]
impl<F: Field> Constraint<F> {
    pub fn evaluate(&self, wire_values: &WireValues<F>) -> bool {
        let a_value = self.a.evaluate(wire_values);
        let b_value = self.b.evaluate(wire_values);
        let c_value = self.c.evaluate(wire_values);
        a_value * b_value == c_value
    }
}
