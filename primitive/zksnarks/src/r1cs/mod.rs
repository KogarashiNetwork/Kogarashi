#![allow(dead_code)]
mod constraint;
mod constraint_system;
pub(crate) mod curves;
mod error;
mod expression;
mod field_arithmetic;
mod prover;
mod util;
pub mod wire;

#[cfg(test)]
mod tests {
    use crate::r1cs::constraint_system::{Circuit, ConstraintSystem};
    use crate::r1cs::error::R1CSError;
    use crate::r1cs::expression::Expression;
    use bls_12_381::Fr as BlsScalar;
    use jub_jub::JubjubAffine;
    use zkstd::common::Field;

    #[test]
    fn circuit_to_r1cs() {
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
            fn synthesize(
                &self,
                composer: &mut ConstraintSystem<BlsScalar>,
            ) -> Result<(), R1CSError> {
                let x = composer.alloc_private(self.x);
                let y = composer.alloc_private(self.y);

                composer.append_edwards_expression::<JubjubAffine>(
                    Expression::from(x),
                    Expression::from(y),
                );

                Ok(())
            }
        }

        let x = BlsScalar::from_hex(
            "0x187d2619ff114316d237e86684fb6e3c6b15e9b924fa4e322764d3177508297a",
        )
        .unwrap();
        let y = BlsScalar::from_hex(
            "0x6230c613f1b460e026221be21cf4eabd5a8ea552db565cb18d3cabc39761eb9b",
        )
        .unwrap();

        let builder = ConstraintSystem::<BlsScalar>::new();
        let circuit = DummyCircuit::new(x, y);

        let mut prover = builder.build(&circuit);
        assert!(prover.prove());
    }
}
