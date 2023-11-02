use crate::groth16::wire::Wire;

use zkstd::common::PrimeField;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Element<F: PrimeField>(pub(crate) Wire, pub(crate) F);

impl<F: PrimeField> Element<F> {
    pub(crate) fn one() -> Self {
        Self(Wire::ONE, F::one())
    }
}
