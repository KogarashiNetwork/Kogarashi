use crate::r1cs::constraint::Constraint;
use crate::r1cs::wire_values::WireValues;
use crate::r1cs::witness_generator::WitnessGenerator;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use zkstd::common::Field;

#[allow(dead_code)]
/// An R1CS gadget.
pub struct Gadget<F: Field> {
    /// The set of rank-1 constraints which define the R1CS instance.
    pub constraints: Vec<Constraint<F>>,
    /// The set of generators used to generate a complete witness from inputs.
    pub witness_generators: Vec<WitnessGenerator<F>>,
}

#[allow(dead_code)]
impl<F: Field> Gadget<F> {
    /// Execute the gadget, and return whether all constraints were satisfied.
    pub fn execute(&self, wire_values: &mut WireValues<F>) -> bool {
        let mut pending_generators: Vec<&WitnessGenerator<F>> =
            self.witness_generators.iter().collect();

        // TODO: This repeatedly enumerates all generators, whether or not any of their dependencies
        // have been generated. A better approach would be to create a map from wires to generators
        // which depend on those wires. Then when a wire is assigned a value, we could efficiently
        // check for generators which are now ready to run, and place them in a queue.
        loop {
            let mut made_progress = false;
            pending_generators.retain(|generator| {
                if wire_values.contains_all(generator.inputs()) {
                    generator.generate(wire_values);
                    made_progress = true;
                    false
                } else {
                    true
                }
            });

            if !made_progress {
                break;
            }
        }

        assert_eq!(
            pending_generators.len(),
            0,
            "Some generators never received inputs"
        );

        self.constraints
            .iter()
            .all(|constraint| constraint.evaluate(wire_values))
    }
}

#[cfg(test)]
mod tests {
    use crate::r1cs::expression::Expression;
    use crate::r1cs::gadget_builder::GadgetBuilder;
    use crate::r1cs::wire_values::WireValues;
    use crate::values;
    use jub_jub::Fr;

    #[test]
    fn constraint_not_satisfied() {
        let mut builder = GadgetBuilder::<Fr>::new();
        let (x, y) = (builder.wire(), builder.wire());
        builder.assert_equal(&Expression::from(x), &Expression::from(y));
        let gadget = builder.build();

        let mut values = values!(x => 42u64.into(), y => 43u64.into());
        let constraints_satisfied = gadget.execute(&mut values);
        assert!(!constraints_satisfied);
    }

    #[test]
    #[should_panic]
    fn missing_generator() {
        let mut builder = GadgetBuilder::<Fr>::new();
        let (x, y, z) = (builder.wire(), builder.wire(), builder.wire());
        builder.assert_product(
            &Expression::from(x),
            &Expression::from(y),
            &Expression::from(z),
        );
        let gadget = builder.build();

        let mut values = values!(x => 2u64.into(), y => 3u64.into());
        gadget.execute(&mut values);
    }

    #[test]
    #[should_panic]
    fn missing_input() {
        let mut builder = GadgetBuilder::<Fr>::new();
        let x = builder.wire();
        builder.inverse(&Expression::from(x));
        let gadget = builder.build();

        let mut values = WireValues::new();
        gadget.execute(&mut values);
    }
}
