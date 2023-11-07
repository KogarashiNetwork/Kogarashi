mod matrix;
mod wire;

pub use matrix::*;
pub use wire::Wire;

use core::ops::Index;
use zkstd::common::PrimeField;

#[derive(Debug)]
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

    pub fn append(&mut self, a: SparseRow<F>, b: SparseRow<F>, c: SparseRow<F>) {
        self.a.0.push(a);
        self.b.0.push(b);
        self.c.0.push(c);
        self.m += 1;
    }

    pub fn alloc_instance(&mut self, instance: F) -> Wire {
        let wire = self.public_wire();
        self.x.push(instance);
        wire
    }

    pub fn alloc_witness(&mut self, witness: F) -> Wire {
        let wire = self.private_wire();
        self.w.push(witness);
        wire
    }

    pub fn constrain_mul(&mut self, x: SparseRow<F>, y: SparseRow<F>, z: SparseRow<F>) {
        self.append(x, y, z)
    }

    pub fn constrain_add(&mut self, x: SparseRow<F>, y: SparseRow<F>, z: SparseRow<F>) {
        self.append(x + y, SparseRow::from(Wire::ONE), z)
    }

    pub fn w(&self) -> DenseVectors<F> {
        self.w.clone()
    }

    pub fn x(&self) -> DenseVectors<F> {
        self.x.clone()
    }

    fn public_wire(&self) -> Wire {
        Wire::Instance(self.x.len())
    }

    fn private_wire(&self) -> Wire {
        Wire::Witness(self.w.len())
    }

    pub fn evaluate(&self, x: &DenseVectors<F>, w: &DenseVectors<F>) -> (Vec<F>, Vec<F>, Vec<F>) {
        let a_evals = self.a.evaluate_with_z(x, w);
        let b_evals = self.b.evaluate_with_z(x, w);
        let c_evals = self.c.evaluate_with_z(x, w);
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
            w: DenseVectors::new(vec![F::one()]),
            x: DenseVectors::default(),
        }
    }
}

impl<F: PrimeField> Index<Wire> for R1cs<F> {
    type Output = F;

    fn index(&self, w: Wire) -> &Self::Output {
        match w {
            Wire::Witness(i) => &self.w[i],
            Wire::Instance(i) => &self.x[i],
        }
    }
}
