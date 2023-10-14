mod constraint;
mod expression;
mod params;
mod prover;
mod util;

pub(crate) mod curves;
pub(crate) mod error;
pub mod wire;

use crate::circuit::Circuit;
use crate::constraint_system::ConstraintSystem;

use constraint::Constraint;
use curves::EdwardsExpression;
use expression::Expression;
use hashbrown::HashMap;
use prover::Prover;
use wire::{Index, Wire};
use zkstd::common::{Group, Ring, TwistedEdwardsAffine, Vec};

pub struct Groth16<C: TwistedEdwardsAffine> {
    constraints: Vec<Constraint<C::Range>>,
    pub(crate) instance: HashMap<Wire, C::Range>,
    pub(crate) witness: HashMap<Wire, C::Range>,
}

impl<C: TwistedEdwardsAffine> ConstraintSystem<C> for Groth16<C> {
    fn new() -> Self {
        Self {
            constraints: Vec::new(),
            instance: HashMap::new(),
            witness: [(Wire::ONE, C::Range::one())].into_iter().collect(),
        }
    }

    fn m(&self) -> usize {
        self.constraints.len()
    }

    fn instance(&self) -> Vec<<C>::Range> {
        Vec::new()
    }

    fn alloc_instance(&mut self, instance: C::Range) -> Wire {
        let wire = self.public_wire();
        self.instance.insert(wire, instance);
        wire
    }

    fn alloc_witness(&mut self, witness: C::Range) -> Wire {
        let wire = self.private_wire();
        self.witness.insert(wire, witness);
        wire
    }
}

impl<C: TwistedEdwardsAffine> Groth16<C> {
    /// Add a public wire to the gadget. It will start with no generator and no associated constraints.
    pub fn public_wire(&mut self) -> Wire {
        let index = self.instance.len();
        Wire::new_unchecked(Index::Input(index))
    }

    /// Add a private wire to the gadget. It will start with no generator and no associated constraints.
    fn private_wire(&mut self) -> Wire {
        let index = self.witness.len();
        Wire::new_unchecked(Index::Aux(index))
    }

    pub fn append_edwards_expression(
        &mut self,
        x: Expression<C::Range>,
        y: Expression<C::Range>,
    ) -> EdwardsExpression<C::Range, C> {
        let x_squared = self.product(&x, &x);
        let y_squared = self.product(&y, &y);
        let x_squared_y_squared = self.product(&x_squared, &y_squared);

        self.assert_equal(
            &y_squared,
            &(Expression::one() + x_squared_y_squared * C::PARAM_D + &x_squared),
        );

        EdwardsExpression::new_unsafe(x, y)
    }

    /// Assert that x * y = z;
    pub fn assert_product(
        &mut self,
        x: &Expression<C::Range>,
        y: &Expression<C::Range>,
        z: &Expression<C::Range>,
    ) {
        self.constraints.push(Constraint {
            a: x.clone(),
            b: y.clone(),
            c: z.clone(),
        });
    }

    /// Assert that x == y.
    pub fn assert_equal(&mut self, x: &Expression<C::Range>, y: &Expression<C::Range>) {
        self.assert_product(x, &Expression::one(), y);
    }

    /// Builds the gadget.
    pub fn build<A>(mut self, circuit: &A) -> Prover<C::Range>
    where
        A: Circuit<C, ConstraintSystem = Self>,
    {
        circuit.synthesize(&mut self).unwrap();

        Prover {
            constraints: self.constraints,
            instance: self.instance,
            witness: self.witness,
        }
    }

    /// The product of two `Expression`s `x` and `y`, i.e. `x * y`.
    pub fn product(
        &mut self,
        x: &Expression<C::Range>,
        y: &Expression<C::Range>,
    ) -> Expression<C::Range> {
        if let Some(c) = x.as_constant() {
            return y * c;
        }
        if let Some(c) = y.as_constant() {
            return x * c;
        }

        let product_value =
            x.evaluate(&self.instance, &self.witness) * y.evaluate(&self.instance, &self.witness);
        let product = self.alloc_instance(product_value);
        let product_exp = Expression::from(product);
        self.assert_product(x, y, &product_exp);

        product_exp
    }

    /// Returns `1 / x`, assuming `x` is non-zero. If `x` is zero, the gadget will not be
    /// satisfiable.
    pub fn inverse(&mut self, x: &Expression<C::Range>) -> Expression<C::Range> {
        let x_value = x.evaluate(&self.instance, &self.witness);
        let inverse_value = x_value.invert().expect("Can't find an inverse element");
        let x_inv = self.alloc_instance(inverse_value);

        let x_inv_expression = Expression::from(x_inv);
        self.assert_product(x, &x_inv_expression, &Expression::one());

        x_inv_expression
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::circuit::Circuit;
    use crate::constraint_system::ConstraintSystem;
    use crate::error::Error;
    use bls_12_381::Fr as BlsScalar;
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
            type ConstraintSystem = Groth16<JubjubAffine>;
            fn synthesize(&self, composer: &mut Groth16<JubjubAffine>) -> Result<(), Error> {
                let x = composer.alloc_witness(self.x);
                let y = composer.alloc_witness(self.y);

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

        let builder = Groth16::<JubjubAffine>::new();
        let circuit = DummyCircuit::new(x, y);

        let mut prover = builder.build(&circuit);
        assert!(prover.create_proof());
    }
}
