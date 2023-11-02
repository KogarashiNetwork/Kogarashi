use crate::groth16::wire::Wire;

use zkstd::common::PrimeField;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Element<F: PrimeField>(pub(crate) Wire, pub(crate) F);
