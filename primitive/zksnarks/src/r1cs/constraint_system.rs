use crate::r1cs::wire::Index;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use core::fmt::Debug;
use zkstd::common::{Field, TwistedEdwardsAffine};

use crate::r1cs::constraint::Constraint;
use crate::r1cs::curves::EdwardsExpression;
use crate::r1cs::error::R1CSError;
use crate::r1cs::expression::Expression;
use crate::r1cs::prover::Prover;
use crate::r1cs::wire::Wire;
use crate::r1cs::wire_values::WireValues;

/// Circuit implementation that can be proved by a Composer
///
/// The default implementation will be used to generate the proving arguments.
pub trait Circuit<F: Field>: Default + Debug {
    /// Circuit definition
    fn synthesize(&self, composer: &mut ConstraintSystem<F>) -> Result<(), R1CSError>;
}

pub struct ConstraintSystem<F: Field> {
    next_wire_index: u32,
    constraints: Vec<Constraint<F>>,
    // witness_generators: Vec<WitnessGenerator<F>>,
    pub(crate) wire_values: WireValues<F>,
}

/// A utility for building `Gadget`s. See the readme for examples.
#[allow(clippy::new_without_default)]
impl<F: Field> ConstraintSystem<F> {
    /// Creates a new `GadgetBuilder`, starting with no constraints or generators.
    pub fn new() -> Self {
        ConstraintSystem {
            next_wire_index: 1,
            constraints: Vec::new(),
            wire_values: WireValues::new(),
        }
    }

    pub fn alloc_public<P: Into<F>>(&mut self, public: P) -> Wire {
        let wire = self.public_wire();
        self.wire_values.set(wire, public.into());
        wire
    }

    /// Add a public wire to the gadget. It will start with no generator and no associated constraints.
    pub fn public_wire(&mut self) -> Wire {
        let index = self.next_wire_index;
        self.next_wire_index += 1;
        Wire::new_unchecked(Index::Input(index as usize))
    }

    pub fn alloc_private<P: Into<F>>(&mut self, private: P) -> Wire {
        let wire = self.private_wire();
        self.wire_values.set(wire, private.into());
        wire
    }

    /// Add a private wire to the gadget. It will start with no generator and no associated constraints.
    fn private_wire(&mut self) -> Wire {
        let index = self.next_wire_index;
        self.next_wire_index += 1;
        Wire::new_unchecked(Index::Aux(index as usize))
    }

    pub fn append_edwards_expression<C: TwistedEdwardsAffine<Range = F>>(
        &mut self,
        x: Expression<F>,
        y: Expression<F>,
    ) -> EdwardsExpression<F, C> {
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
    pub fn build<C>(mut self, circuit: &C) -> Prover<F>
    where
        C: Circuit<F>,
    {
        circuit.synthesize(&mut self).unwrap();
        Prover {
            constraints: self.constraints,
            wire_values: self.wire_values,
        }
    }
}
