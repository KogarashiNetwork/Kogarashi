#![doc = include_str!("../README.md")]

pub mod gadget;
mod matrix;
#[cfg(test)]
mod test;
mod wire;

pub use matrix::{DenseVectors, SparseMatrix, SparseRow};
pub use wire::Wire;

use core::ops::Index;
use zkstd::common::{vec, PrimeField, Vec};

#[derive(Clone, Debug)]
pub struct R1cs<F: PrimeField> {
    // 1. Structure S
    // a, b and c matrices and matrix size
    m: usize,
    a: SparseMatrix<F>,
    b: SparseMatrix<F>,
    c: SparseMatrix<F>,

    // 2. Instance
    // r1cs witness includes private inputs and intermediate value
    w: DenseVectors<F>,

    // 3. Witness
    // r1cs instance includes public inputs and outputs
    x: DenseVectors<F>,
}

impl<F: PrimeField> R1cs<F> {
    pub fn m(&self) -> usize {
        self.m
    }

    pub fn l(&self) -> usize {
        self.x.len()
    }

    pub fn m_l_1(&self) -> usize {
        self.w.len()
    }

    pub fn x(&self) -> Vec<F> {
        self.x.get()
    }

    pub fn w(&self) -> Vec<F> {
        self.w.get()
    }

    ///  check (A · Z) ◦ (B · Z) = C · Z
    pub fn is_sat(&self) -> bool {
        let R1cs { m, a, b, c, x, w } = self;
        // A · Z
        let az = a.prod(m, x, w);
        // B · Z
        let bz = b.prod(m, x, w);
        // C · Z
        let cz = c.prod(m, x, w);
        // (A · Z) ◦ (B · Z)
        let azbz = az * bz;

        azbz.iter()
            .zip(cz.iter())
            .all(|(left, right)| left == right)
    }

    fn append(&mut self, a: SparseRow<F>, b: SparseRow<F>, c: SparseRow<F>) {
        self.a.0.push(a);
        self.b.0.push(b);
        self.c.0.push(c);
        self.m += 1;
    }

    fn public_wire(&mut self) -> Wire {
        let index = self.x.len();
        Wire::Instance(index)
    }

    fn private_wire(&mut self) -> Wire {
        let index = self.w.len();
        Wire::Witness(index)
    }

    /// constrain x * y = z
    pub fn mul_gate(&mut self, x: &SparseRow<F>, y: &SparseRow<F>, z: &SparseRow<F>) {
        self.append(x.clone(), y.clone(), z.clone());
    }

    /// constrain x + y = z
    pub fn add_gate(&mut self, x: &SparseRow<F>, y: &SparseRow<F>, z: &SparseRow<F>) {
        self.append(x + y, SparseRow::from(Wire::ONE), z.clone());
    }

    /// constrain x == y
    pub fn equal_gate(&mut self, x: &SparseRow<F>, y: &SparseRow<F>) {
        self.mul_gate(x, &SparseRow::one(), y);
    }

    pub fn evaluate(&self) -> (Vec<F>, Vec<F>, Vec<F>) {
        let a_evals = self.a.evaluate_with_z(&self.x, &self.w);
        let b_evals = self.b.evaluate_with_z(&self.x, &self.w);
        let c_evals = self.c.evaluate_with_z(&self.x, &self.w);
        (a_evals, b_evals, c_evals)
    }

    pub fn z_vectors(
        &self,
        l: usize,
        m_l_1: usize,
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
        let (a_x, a_w) = self.a.x_and_w(l, m_l_1);
        let (b_x, b_w) = self.b.x_and_w(l, m_l_1);
        let (c_x, c_w) = self.c.x_and_w(l, m_l_1);

        ((a_x, b_x, c_x), (a_w, b_w, c_w))
    }
}

impl<F: PrimeField> Default for R1cs<F> {
    fn default() -> Self {
        Self {
            m: 0,
            a: SparseMatrix::default(),
            b: SparseMatrix::default(),
            c: SparseMatrix::default(),
            x: DenseVectors::new(vec![F::one()]),
            w: DenseVectors::default(),
        }
    }
}

impl<F: PrimeField> Index<Wire> for R1cs<F> {
    type Output = F;

    fn index(&self, w: Wire) -> &Self::Output {
        match w {
            Wire::Instance(i) => &self.x[i],
            Wire::Witness(i) => &self.w[i],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test::example_r1cs;
    use jub_jub::Fr as Scalar;

    #[test]
    fn r1cs_test() {
        for i in 1..10 {
            let r1cs = example_r1cs::<Scalar>(i);
            assert!(r1cs.is_sat())
        }
    }
}
