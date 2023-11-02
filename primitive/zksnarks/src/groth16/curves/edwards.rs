use crate::groth16::matrix::{Element, Evaluable, SparseRow};
use core::marker::PhantomData;
use zkstd::common::{PrimeField, TwistedEdwardsAffine};

impl<F: PrimeField, C: TwistedEdwardsAffine<Range = F>> Clone for EdwardsExpression<F, C> {
    fn clone(&self) -> Self {
        EdwardsExpression {
            x: self.x.clone(),
            y: self.y.clone(),
            marker: self.marker,
        }
    }
}

pub struct EdwardsExpression<F: PrimeField, C: TwistedEdwardsAffine<Range = F>> {
    pub x: SparseRow<F>,
    pub y: SparseRow<F>,
    marker: PhantomData<C>,
}

impl<F: PrimeField, C: TwistedEdwardsAffine<Range = F>> EdwardsExpression<F, C> {
    pub fn identity() -> Self {
        Self::new_unsafe(
            SparseRow::from(C::Range::zero()),
            SparseRow::from(C::Range::one()),
        )
    }

    /// Creates an `EdwardsExpression` from two arbitrary coordinates of type `Expression`.
    /// This method is unsafe and should only be used when the coordinates are proven
    /// to exist on the curve.
    pub fn new_unsafe(x: SparseRow<C::Range>, y: SparseRow<C::Range>) -> EdwardsExpression<F, C> {
        EdwardsExpression {
            x,
            y,
            marker: Default::default(),
        }
    }
}

impl<F: PrimeField, C: TwistedEdwardsAffine<Range = F>> Evaluable<F, C>
    for EdwardsExpression<F, C>
{
    fn evaluate(&self, instance: &[Element<F>], witness: &[Element<F>]) -> C {
        C::from_raw_unchecked(
            self.x.evaluate(instance, witness),
            self.y.evaluate(instance, witness),
        )
    }
}
