use super::constraint::Constraint;
use super::wire::Wire;
use hashbrown::HashMap;
use zkstd::common::{Field, Vec};

/// An R1CS gadget.
pub struct Prover<F: Field> {
    /// The set of rank-1 constraints which define the R1CS instance.
    pub constraints: Vec<Constraint<F>>,
    pub(crate) instance: HashMap<Wire, F>,
    pub(crate) witness: HashMap<Wire, F>,
}

impl<F: Field> Prover<F> {
    /// Execute the gadget, and return whether all constraints were satisfied.
    pub fn prove(&mut self) -> bool {
        self.constraints
            .iter()
            .all(|constraint| constraint.evaluate(&self.instance, &self.witness))
    }
}

#[cfg(test)]
mod tests {
    use crate::circuit::Circuit;
    use crate::error::Error;
    use crate::groth16::constraint_system::ConstraintSystem;
    use crate::groth16::expression::Expression;
    use bls_12_381::Fr as BlsScalar;
    use zkstd::common::Field;

    #[test]
    fn constraint_satisfied() {
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
            fn synthesize(&self, composer: &mut ConstraintSystem<BlsScalar>) -> Result<(), Error> {
                let (x, y) = (composer.alloc_public(self.x), composer.alloc_public(self.y));
                composer.assert_equal(&Expression::from(x), &Expression::from(y));

                Ok(())
            }
        }

        let builder = ConstraintSystem::<BlsScalar>::new();
        let circuit = DummyCircuit::new(43u64.into(), 43u64.into());

        let mut prover = builder.build(&circuit);

        assert!(prover.prove());
    }

    #[test]
    fn constraint_not_satisfied() {
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
            fn synthesize(&self, composer: &mut ConstraintSystem<BlsScalar>) -> Result<(), Error> {
                let (x, y) = (composer.alloc_public(self.x), composer.alloc_public(self.y));
                composer.assert_equal(&Expression::from(x), &Expression::from(y));

                Ok(())
            }
        }

        let builder = ConstraintSystem::<BlsScalar>::new();
        let circuit = DummyCircuit::new(42u64.into(), 43u64.into());

        let mut prover = builder.build(&circuit);

        let constraints_satisfied = prover.prove();
        assert!(!constraints_satisfied);
    }
}
