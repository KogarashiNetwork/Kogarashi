use super::expression::Expression;
use super::wire::Wire;

use hashbrown::HashMap;
use zkstd::common::Field;

/// An rank-1 constraint of the form a * b = c, where a, b, and c are linear combinations of wires.
#[derive(Clone, Debug)]
pub struct Constraint<F: Field> {
    pub a: Expression<F>,
    pub b: Expression<F>,
    pub c: Expression<F>,
}

impl<F: Field> Constraint<F> {
    pub fn evaluate(&self, instance: &HashMap<Wire, F>, witness: &HashMap<Wire, F>) -> bool {
        let a_value = self.a.evaluate(instance, witness);
        let b_value = self.b.evaluate(instance, witness);
        let c_value = self.c.evaluate(instance, witness);
        a_value * b_value == c_value
    }
}
