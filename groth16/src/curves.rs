use r1cs::{Evaluable, SparseRow, Wire};
use zkstd::common::{IntGroup, Ring, TwistedEdwardsAffine};

#[derive(Clone, Debug)]
pub struct CurveWitness<C: TwistedEdwardsAffine> {
    pub x: SparseRow<C::Range>,
    pub y: SparseRow<C::Range>,
}

impl<C: TwistedEdwardsAffine> Evaluable<C::Range, C> for CurveWitness<C> {
    fn evaluate(&self, instance: &[(Wire, C::Range)], witness: &[(Wire, C::Range)]) -> C {
        C::from_raw_unchecked(
            self.x.evaluate(instance, witness),
            self.y.evaluate(instance, witness),
        )
    }
}

impl<C: TwistedEdwardsAffine> CurveWitness<C> {
    pub fn identity() -> Self {
        Self::new_unsafe(
            SparseRow::from(C::Range::zero()),
            SparseRow::from(C::Range::one()),
        )
    }

    /// Creates an `CurveWitness` from two arbitrary coordinates of type `Expression`.
    /// This method is unsafe and should only be used when the coordinates are proven
    /// to exist on the curve.
    pub fn new_unsafe(x: SparseRow<C::Range>, y: SparseRow<C::Range>) -> CurveWitness<C> {
        Self { x, y }
    }
}
