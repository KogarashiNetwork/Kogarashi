use crate::groth16::wire_alt::Wire;

use zkstd::common::PrimeField;

#[derive(Clone, Debug)]
pub(crate) struct Element<F: PrimeField>(pub(crate) Wire, pub(crate) F);
