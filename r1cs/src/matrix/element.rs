use crate::wire::Wire;

use zkstd::common::PrimeField;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Element<F: PrimeField>(pub Wire, pub F);

impl<F: PrimeField> Element<F> {
    pub fn one() -> Self {
        Self(Wire::ONE, F::one())
    }
}

pub trait Evaluable<F: PrimeField, R> {
    fn evaluate(&self, instance: &[Element<F>], witness: &[Element<F>]) -> R;
}
