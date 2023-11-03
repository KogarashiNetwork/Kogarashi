use crate::groth16::matrix::{Element, Evaluable, SparseRow};

use core::marker::PhantomData;
use zkstd::common::{Group, Ring, TwistedEdwardsAffine};

impl<C: TwistedEdwardsAffine> Clone for EdwardsExpression<C> {
    fn clone(&self) -> Self {
        EdwardsExpression {
            x: self.x.clone(),
            y: self.y.clone(),
            marker: self.marker,
        }
    }
}

pub struct EdwardsExpression<C: TwistedEdwardsAffine> {
    pub x: SparseRow<C::Range>,
    pub y: SparseRow<C::Range>,
    marker: PhantomData<C>,
}

impl<C: TwistedEdwardsAffine> EdwardsExpression<C> {
    pub fn identity() -> Self {
        Self::new_unsafe(
            SparseRow::from(C::Range::zero()),
            SparseRow::from(C::Range::one()),
        )
    }

    /// Creates an `EdwardsExpression` from two arbitrary coordinates of type `Expression`.
    /// This method is unsafe and should only be used when the coordinates are proven
    /// to exist on the curve.
    pub fn new_unsafe(x: SparseRow<C::Range>, y: SparseRow<C::Range>) -> EdwardsExpression<C> {
        EdwardsExpression {
            x,
            y,
            marker: Default::default(),
        }
    }
}

impl<C: TwistedEdwardsAffine> Evaluable<C::Range, C> for EdwardsExpression<C> {
    fn evaluate(&self, instance: &[Element<C::Range>], witness: &[Element<C::Range>]) -> C {
        C::from_raw_unchecked(
            self.x.evaluate(instance, witness),
            self.y.evaluate(instance, witness),
        )
    }
}
