use crate::wire::Wire;

use zkstd::common::PrimeField;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Entry<F: PrimeField>(pub Wire, pub F);

impl<F: PrimeField> Entry<F> {
    pub fn one() -> Self {
        Self(Wire::ONE, F::one())
    }
}

pub trait Evaluable<F: PrimeField, R> {
    fn evaluate(&self, instance: &[Entry<F>], witness: &[Entry<F>]) -> R;
}
