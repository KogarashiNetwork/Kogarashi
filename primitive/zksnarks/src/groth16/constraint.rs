use super::matrix::{Element, SparseMatrix, SparseRow};
use super::wire::Wire;

use zkstd::common::{PrimeField, Vec};

#[derive(Clone, Debug, Default)]
pub struct R1csStruct<F: PrimeField> {
    // matrix size
    m: usize,
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
        instance: &[Element<F>],
        witness: &[Element<F>],
    ) -> (Vec<F>, Vec<F>, Vec<F>) {
        let (mut a_evals, mut b_evals, mut c_evals) = (Vec::new(), Vec::new(), Vec::new());
        self.a
            .0
            .iter()
            .zip(self.b.0.iter())
            .zip(self.c.0.iter())
            .for_each(|((a, b), c)| {
                a_evals.push(a.evaluate(instance, witness));
                b_evals.push(b.evaluate(instance, witness));
                c_evals.push(c.evaluate(instance, witness));
            });
        (a_evals, b_evals, c_evals)
    }

    pub(crate) fn z_vectors(
        &self,
        instance_size: usize,
        witness_size: usize,
    ) -> (
        (
            Vec<Vec<(F, usize)>>,
            Vec<Vec<(F, usize)>>,
            Vec<Vec<(F, usize)>>,
        ),
        (
            Vec<Vec<(F, usize)>>,
            Vec<Vec<(F, usize)>>,
            Vec<Vec<(F, usize)>>,
        ),
    ) {
        let mut a_instance = vec![vec![]; instance_size];
        let mut b_instance = vec![vec![]; instance_size];
        let mut c_instance = vec![vec![]; instance_size];
        let mut a_witness = vec![vec![]; witness_size];
        let mut b_witness = vec![vec![]; witness_size];
        let mut c_witness = vec![vec![]; witness_size];
        for (i, ((a, b), c)) in self
            .a
            .0
            .iter()
            .zip(self.b.0.iter())
            .zip(self.c.0.iter())
            .enumerate()
        {
            a.coefficients()
                .iter()
                .for_each(|Element(w, coeff)| match w {
                    Wire::Instance(k) => a_instance[*k].push((*coeff, i)),
                    Wire::Witness(k) => a_witness[*k].push((*coeff, i)),
                });
            b.coefficients()
                .iter()
                .for_each(|Element(w, coeff)| match w {
                    Wire::Instance(k) => b_instance[*k].push((*coeff, i)),
                    Wire::Witness(k) => b_witness[*k].push((*coeff, i)),
                });
            c.coefficients()
                .iter()
                .for_each(|Element(w, coeff)| match w {
                    Wire::Instance(k) => c_instance[*k].push((*coeff, i)),
                    Wire::Witness(k) => c_witness[*k].push((*coeff, i)),
                });
        }

        (
            (a_instance, b_instance, c_instance),
            (a_witness, b_witness, c_witness),
        )
    }
}
