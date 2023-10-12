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
    use crate::r1cs::error::R1CSError;
    use crate::r1cs::expression::Expression;
    use crate::r1cs::gadget_builder::{Circuit, GadgetBuilder};
    use crate::r1cs::wire_values::WireValues;
    use bls_12_381::Fr as BlsScalar;
    use zkstd::common::Field;

    #[test]
    fn constraint_not_satisfied() {
        #[allow(dead_code)]
        #[derive(Debug)]
        pub struct DummyCircuit<F: Field> {
            x: F,
            y: F,
        }

        impl DummyCircuit<BlsScalar> {
            pub fn new(x: BlsScalar, y: BlsScalar) -> Self {
                Self { x, y }
            }
        }

        impl Default for DummyCircuit<BlsScalar> {
            fn default() -> Self {
                Self::new(0.into(), 0.into())
            }
        }

        impl Circuit<BlsScalar> for DummyCircuit<BlsScalar> {
            fn circuit(&self, composer: &mut GadgetBuilder<BlsScalar>) -> Result<(), R1CSError> {
                let (x, y) = (composer.public_wire(), composer.public_wire());
                composer.assert_equal(&Expression::from(x), &Expression::from(y));

                Ok(())
            }
        }

        let builder = GadgetBuilder::<BlsScalar>::new();
        let circuit = DummyCircuit::new(42u64.into(), 43u64.into());

        let gadget = builder.build(&circuit);

        // let mut values = values!(x => 42u64.into(), y => 43u64.into());
        let constraints_satisfied = gadget.execute(&mut WireValues::new());
        assert!(!constraints_satisfied);
    }
}
