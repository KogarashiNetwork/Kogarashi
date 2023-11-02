use super::matrix::{Element, SparseMatrix, SparseRow};

use zkstd::common::{PrimeField, Vec};

#[derive(Clone, Debug, Default)]
pub struct R1csStruct<F: PrimeField> {
    // matrix size
    m: usize,
    // public input size
    l: usize,
    pub(crate) a: SparseMatrix<F>,
    pub(crate) b: SparseMatrix<F>,
    pub(crate) c: SparseMatrix<F>,
}

impl<F: PrimeField> R1csStruct<F> {
    pub(crate) fn m(&self) -> usize {
        self.m
    }

    pub(crate) fn append(&mut self, a: SparseRow<F>, b: SparseRow<F>, c: SparseRow<F>) {
        self.a.0.push(a);
        self.b.0.push(b);
        self.c.0.push(c);
        self.m += 1;
    }

    pub(crate) fn evaluate(
        &self,
        instance: &Vec<Element<F>>,
        witness: &Vec<Element<F>>,
    ) -> (Vec<F>, Vec<F>, Vec<F>) {
        let (mut a_evals, mut b_evals, mut c_evals) = (Vec::new(), Vec::new(), Vec::new());
        self.a
            .0
            .iter()
            .zip(self.b.0.iter())
            .zip(self.c.0.iter())
            .for_each(|((a, b), c)| {
                a_evals.push(a.evaluate(&instance, &witness));
                b_evals.push(b.evaluate(&instance, &witness));
                c_evals.push(c.evaluate(&instance, &witness));
            });
        (a_evals, b_evals, c_evals)
    }
}
