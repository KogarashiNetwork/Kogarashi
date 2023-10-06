use crate::r1cs::wire_values::WireValues;
#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use zkstd::common::Field;

use crate::wire::Wire;

#[allow(clippy::type_complexity)]
/// Generates some elements of the witness.
pub struct WitnessGenerator<F: Field> {
    inputs: Vec<Wire>,
    generator: Box<dyn Fn(&mut WireValues<F>)>,
}

#[allow(dead_code)]
impl<F: Field> WitnessGenerator<F> {
    /// Creates a new `WitnessGenerator`.
    ///
    /// # Arguments
    /// * `inputs` - the wires whose values must be set before this generator can run
    /// * `generate` - a function which generates some elements of the witness
    pub fn new<T>(inputs: Vec<Wire>, generate: T) -> Self
    where
        T: Fn(&mut WireValues<F>) + 'static,
    {
        WitnessGenerator {
            inputs,
            generator: Box::new(generate),
        }
    }

    /// The wires whose values must be set before this generator can run.
    pub fn inputs(&self) -> &[Wire] {
        &self.inputs
    }

    /// Run the generator.
    pub fn generate(&self, values: &mut WireValues<F>) {
        (*self.generator)(values)
    }
}
