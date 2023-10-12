mod constraint;
pub(crate) mod curves;
mod error;
mod expression;
mod field_arithmetic;
mod gadget;
mod gadget_builder;
mod util;
pub mod wire;
mod wire_values;
mod witness_generator;

#[cfg(test)]
mod tests {
    use crate::r1cs::error::R1CSError;
    use crate::r1cs::expression::Expression;
    use crate::r1cs::gadget_builder::{Circuit, GadgetBuilder};
    use crate::r1cs::wire_values::WireValues;
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
            fn circuit(&self, composer: &mut GadgetBuilder<BlsScalar>) -> Result<(), R1CSError> {
                let x_exp = Expression::from(self.x);
                let y_exp = Expression::from(self.y);

                composer.append_edwards_expression::<JubjubAffine>(x_exp, y_exp);

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

        let builder = GadgetBuilder::<BlsScalar>::new();
        let circuit = DummyCircuit::new(x, y);

        let gadget = builder.build(&circuit);
        assert!(gadget.execute(&mut WireValues::new()));
    }
}
