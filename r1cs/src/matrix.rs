mod row;
mod vector;

use crate::wire::Wire;
pub use row::SparseRow;
pub use vector::DenseVectors;

use zkstd::common::{Debug, PrimeField, Vec};

#[derive(Clone, Debug, Default)]
pub struct SparseMatrix<F: PrimeField>(pub(crate) Vec<SparseRow<F>>);

impl<F: PrimeField> SparseMatrix<F> {
    pub(crate) fn x_and_w(
        &self,
        l: usize,
        m_l_1: usize,
    ) -> (Vec<Vec<(F, usize)>>, Vec<Vec<(F, usize)>>) {
        let mut x = vec![vec![]; l];
        let mut w = vec![vec![]; m_l_1];
        for (i, a) in self.0.iter().enumerate() {
            a.iter().for_each(|(wire, coeff)| match wire {
                Wire::Instance(k) => x[*k].push((*coeff, i)),
                Wire::Witness(k) => w[*k].push((*coeff, i)),
            });
        }
        (x, w)
    }

    pub(crate) fn evaluate_with_z(&self, x: &DenseVectors<F>, w: &DenseVectors<F>) -> Vec<F> {
        self.0.iter().map(|row| row.evaluate(x, w)).collect()
    }
}

pub trait Evaluable<F: PrimeField, R> {
    fn evaluate(&self, instance: &DenseVectors<F>, witness: &DenseVectors<F>) -> R;
}
