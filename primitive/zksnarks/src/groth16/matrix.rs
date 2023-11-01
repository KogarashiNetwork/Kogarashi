mod element;

pub(crate) use element::Element;
use zkstd::common::{PrimeField, Vec};

#[derive(Clone, Debug, Default)]
pub(crate) struct SparseMatrix<F: PrimeField>(pub(crate) Vec<Vec<Element<F>>>);