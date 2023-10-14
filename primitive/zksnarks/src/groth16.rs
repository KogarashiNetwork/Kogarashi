mod constraint;
pub(crate) mod constraint_system;
pub(crate) mod curves;
pub(crate) mod error;
mod expression;
mod field_arithmetic;
mod params;
mod prover;
mod util;
pub mod wire;
pub use params::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::circuit::Circuit;
    use crate::error::Error;
    use bls_12_381::Fr as BlsScalar;
    use constraint_system::ConstraintSystem;
    use expression::Expression;
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

        impl Circuit<JubjubAffine> for DummyCircuit<BlsScalar> {
            fn synthesize(
                &self,
                composer: &mut ConstraintSystem<JubjubAffine>,
            ) -> Result<(), Error> {
                let x = composer.alloc_private(self.x);
                let y = composer.alloc_private(self.y);

                composer.append_edwards_expression(Expression::from(x), Expression::from(y));

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

        let builder = ConstraintSystem::<JubjubAffine>::new();
        let circuit = DummyCircuit::new(x, y);

        let mut prover = builder.build(&circuit);
        assert!(prover.prove());
    }
}
