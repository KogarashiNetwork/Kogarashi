mod field;

use crate::matrix::SparseRow;
use crate::R1cs;

use zkstd::common::PrimeField;

impl<F: PrimeField> R1cs<F> {
    /// product of two SparseRow x and y
    pub fn product(&mut self, x: &SparseRow<F>, y: &SparseRow<F>) -> SparseRow<F> {
        if let Some(c) = x.as_constant() {
            return y * c;
        }
        if let Some(c) = y.as_constant() {
            return x * c;
        }

        let product_value = x.evaluate(&self.x, &self.w) * y.evaluate(&self.x, &self.w);
        let product = self.alloc_witness(product_value);
        let product_exp = SparseRow::from(product);
        self.constrain_mul(x, y, &product_exp);

        product_exp
    }

    /// sum of two SparseRow x and y
    pub fn sum(&mut self, x: &SparseRow<F>, y: &SparseRow<F>) -> SparseRow<F> {
        if let Some(c) = x.as_constant() {
            return y * c;
        }
        if let Some(c) = y.as_constant() {
            return x * c;
        }

        let sum_value = x.evaluate(&self.x, &self.w) + y.evaluate(&self.x, &self.w);
        let sum = self.alloc_witness(sum_value);
        let sum_exp = SparseRow::from(sum);
        self.constrain_add(x, y, &sum_exp);
        sum_exp
    }
}
