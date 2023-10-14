use super::constraint::Constraint;
use super::curves::EdwardsExpression;
use super::expression::Expression;
use super::prover::Prover;
use super::wire::Index;
use super::wire::Wire;
use crate::circuit::Circuit;
use hashbrown::HashMap;
use zkstd::common::{Ring, TwistedEdwardsAffine, Vec};

pub struct ConstraintSystem<C: TwistedEdwardsAffine> {
    constraints: Vec<Constraint<C::Range>>,
    pub(crate) instance: HashMap<Wire, C::Range>,
    pub(crate) witness: HashMap<Wire, C::Range>,
}

/// A utility for building `Gadget`s. See the readme for examples.
#[allow(clippy::new_without_default)]
impl<C: TwistedEdwardsAffine> ConstraintSystem<C> {
    /// Creates a new `GadgetBuilder`, starting with no constraints or generators.
    pub fn new() -> Self {
        ConstraintSystem {
            constraints: Vec::new(),
            instance: HashMap::new(),
            witness: [(Wire::ONE, C::Range::one())].into_iter().collect(),
        }
    }

    pub fn alloc_public<P: Into<C::Range>>(&mut self, public: P) -> Wire {
        let wire = self.public_wire();
        self.instance.insert(wire, public.into());
        wire
    }

    /// Add a public wire to the gadget. It will start with no generator and no associated constraints.
    pub fn public_wire(&mut self) -> Wire {
        let index = self.instance.len();
        Wire::new_unchecked(Index::Input(index))
    }

    pub fn alloc_private<P: Into<C::Range>>(&mut self, private: P) -> Wire {
        let wire = self.private_wire();
        self.witness.insert(wire, private.into());
        wire
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
        A: Circuit<C>,
    {
        circuit.synthesize(&mut self).unwrap();

        Prover {
            constraints: self.constraints,
            instance: self.instance,
            witness: self.witness,
        }
    }
}
