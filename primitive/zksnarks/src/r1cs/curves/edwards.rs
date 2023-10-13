use crate::r1cs::expression::{Evaluable, Expression};
use crate::r1cs::wire::Wire;
use core::marker::PhantomData;
use hashbrown::HashMap;
use zkstd::common::{Field, TwistedEdwardsAffine};

impl<F: Field, C: TwistedEdwardsAffine<Range = F>> Clone for EdwardsExpression<F, C> {
    fn clone(&self) -> Self {
        EdwardsExpression {
            x: self.x.clone(),
            y: self.y.clone(),
            marker: self.marker,
        }
    }
}

pub struct EdwardsExpression<F: Field, C: TwistedEdwardsAffine<Range = F>> {
    pub x: Expression<F>,
    pub y: Expression<F>,
    marker: PhantomData<C>,
}

impl<F: Field, C: TwistedEdwardsAffine<Range = F>> EdwardsExpression<F, C> {
    /// Creates an `EdwardsExpression` from two arbitrary coordinates of type `Expression`.
    /// This method is unsafe and should only be used when the coordinates are proven
    /// to exist on the curve.
    pub fn new_unsafe(x: Expression<C::Range>, y: Expression<C::Range>) -> EdwardsExpression<F, C> {
        EdwardsExpression {
            x,
            y,
            marker: Default::default(),
        }
    }
}

impl<F: Field, C: TwistedEdwardsAffine<Range = F>> Evaluable<F, C> for EdwardsExpression<F, C> {
    fn evaluate(&self, instance: &HashMap<Wire, F>, witness: &HashMap<Wire, F>) -> C {
        C::from_raw_unchecked(
            self.x.evaluate(instance, witness),
            self.y.evaluate(instance, witness),
        )
    }
}
