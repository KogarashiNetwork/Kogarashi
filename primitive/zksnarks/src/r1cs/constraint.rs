use crate::r1cs::matrix::SparseMatrix;
use zkstd::behave::PrimeField;
use zkstd::common::Field;

/// Each gate expression
struct Constraint<const M: usize, F: Field> {
    a: SparseMatrix<M, F>,
    b: SparseMatrix<M, F>,
    c: SparseMatrix<M, F>,
}

impl<const M: usize, F: PrimeField> Constraint<M, F> {}
