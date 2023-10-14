use super::constraint::Constraint;
use super::curves::EdwardsExpression;
use super::expression::Expression;
use super::prover::Prover;
use super::wire::Index;
use super::wire::Wire;
use crate::circuit::Circuit;
use hashbrown::HashMap;
use zkstd::common::{Field, TwistedEdwardsAffine, Vec};

pub struct ConstraintSystem<F: Field> {
    constraints: Vec<Constraint<F>>,
    pub(crate) instance: HashMap<Wire, F>,
    pub(crate) witness: HashMap<Wire, F>,
}

/// A utility for building `Gadget`s. See the readme for examples.
#[allow(clippy::new_without_default)]
impl<F: Field> ConstraintSystem<F> {
    /// Creates a new `GadgetBuilder`, starting with no constraints or generators.
    pub fn new() -> Self {
        ConstraintSystem {
            constraints: Vec::new(),
            instance: HashMap::new(),
            witness: [(Wire::ONE, F::one())].into_iter().collect(),
        }
    }

    pub fn alloc_public<P: Into<F>>(&mut self, public: P) -> Wire {
        let wire = self.public_wire();
        self.instance.insert(wire, public.into());
        wire
    }

    /// Add a public wire to the gadget. It will start with no generator and no associated constraints.
    pub fn public_wire(&mut self) -> Wire {
        let index = self.instance.len();
        Wire::new_unchecked(Index::Input(index))
    }

    pub fn alloc_private<P: Into<F>>(&mut self, private: P) -> Wire {
        let wire = self.private_wire();
        self.witness.insert(wire, private.into());
        wire
    }

    /// Add a private wire to the gadget. It will start with no generator and no associated constraints.
    fn private_wire(&mut self) -> Wire {
        let index = self.witness.len();
        Wire::new_unchecked(Index::Aux(index))
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
            instance: self.instance,
            witness: self.witness,
        }
    }
}
