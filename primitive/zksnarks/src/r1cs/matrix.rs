use zkstd::common::Field;

pub struct SparseMatrix<const M: usize, F: Field>([[F; M]; M]);
