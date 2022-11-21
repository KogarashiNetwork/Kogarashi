use zero_crypto::common::PrimeField;

#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial<F>(pub(crate) Vec<F>);
