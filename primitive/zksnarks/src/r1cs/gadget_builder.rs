#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use zkstd::common::Field;

use crate::r1cs::constraint::Constraint;
use crate::r1cs::expression::Expression;
use crate::r1cs::gadget::Gadget;
use crate::r1cs::wire_values::WireValues;
use crate::r1cs::witness_generator::WitnessGenerator;
use crate::wire::Wire;

pub struct GadgetBuilder<F: Field> {
    next_wire_index: u32,
    constraints: Vec<Constraint<F>>,
    witness_generators: Vec<WitnessGenerator<F>>,
}

#[allow(dead_code)]
/// A utility for building `Gadget`s. See the readme for examples.
#[allow(clippy::new_without_default)]
impl<F: Field> GadgetBuilder<F> {
    /// Creates a new `GadgetBuilder`, starting with no constraints or generators.
    pub fn new() -> Self {
        GadgetBuilder {
            next_wire_index: 1,
            constraints: Vec::new(),
            witness_generators: Vec::new(),
        }
    }

    /// Add a wire to the gadget. It will start with no generator and no associated constraints.
    pub fn wire(&mut self) -> Wire {
        let index = self.next_wire_index;
        self.next_wire_index += 1;
        Wire::new(index as usize)
    }

    /// Add a generator function for setting certain wire values.
    pub fn generator<T>(&mut self, dependencies: Vec<Wire>, generate: T)
    where
        T: Fn(&mut WireValues<F>) + 'static,
    {
        self.witness_generators
            .push(WitnessGenerator::new(dependencies, generate));
    }

    /// Assert that x * y = z;
    pub fn assert_product(&mut self, x: &Expression<F>, y: &Expression<F>, z: &Expression<F>) {
        self.constraints.push(Constraint {
            a: x.clone(),
            b: y.clone(),
            c: z.clone(),
        });
    }

    /// Assert that x == y.
    pub fn assert_equal(&mut self, x: &Expression<F>, y: &Expression<F>) {
        self.assert_product(x, &Expression::one(), y);
    }

    /// Builds the gadget.
    pub fn build(self) -> Gadget<F> {
        Gadget {
            constraints: self.constraints,
            witness_generators: self.witness_generators,
        }
    }
}
