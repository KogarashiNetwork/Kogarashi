use crate::Witness;
use zkstd::common::PrimeField;

pub(crate) struct Constraint<F: PrimeField> {
    selectors: [F; 13],
    witnesses: [Witness; 4],
    has_public_input: bool,
}
