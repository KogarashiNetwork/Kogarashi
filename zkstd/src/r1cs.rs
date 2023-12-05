pub mod test;
mod wire;

use crate::circuit::CircuitDriver;
use crate::common::{vec, Ring, Vec};
use crate::matrix::{DenseVectors, SparseMatrix, SparseRow};

use sp_std::ops::Index;
pub(crate) use wire::Wire;

#[derive(Clone, Debug)]
pub struct R1cs<C: CircuitDriver> {
    // 1. Structure S
    // a, b and c matrices and matrix size
    m: usize,
    a: SparseMatrix<C::Scalar>,
    b: SparseMatrix<C::Scalar>,
    c: SparseMatrix<C::Scalar>,

    // 2. Instance
    // r1cs instance includes one constant and public inputs and outputs
    pub(crate) x: DenseVectors<C::Scalar>,

    // 3. Witness
    // r1cs witness includes private inputs and intermediate value
    pub(crate) w: DenseVectors<C::Scalar>,
}

impl<C: CircuitDriver> R1cs<C> {
    pub fn m(&self) -> usize {
        self.m
    }

    pub fn l(&self) -> usize {
        self.x.len()
    }

    pub fn m_l_1(&self) -> usize {
        self.w.len()
    }

    pub fn x(&self) -> Vec<C::Scalar> {
        self.x.get()
    }

    pub fn w(&self) -> Vec<C::Scalar> {
        self.w.get()
    }

    #[allow(clippy::type_complexity)]
    pub fn matrices(
        &self,
    ) -> (
        SparseMatrix<C::Scalar>,
        SparseMatrix<C::Scalar>,
        SparseMatrix<C::Scalar>,
    ) {
        (self.a.clone(), self.b.clone(), self.c.clone())
    }

    ///  check (A · Z) ◦ (B · Z) = C · Z
    pub fn is_sat(&self) -> bool {
        let R1cs { m, a, b, c, x, w } = self;
        let z = DenseVectors::new(vec![x.get(), w.get()].concat());
        // A · Z
        let az = a.prod(m, self.l(), &z);
        // B · Z
        let bz = b.prod(m, self.l(), &z);
        // C · Z
        let cz = c.prod(m, self.l(), &z);
        // (A · Z) ◦ (B · Z)
        let azbz = az * bz;

        azbz.iter()
            .zip(cz.iter())
            .all(|(left, right)| left == right)
    }

    fn append(
        &mut self,
        a: SparseRow<C::Scalar>,
        b: SparseRow<C::Scalar>,
        c: SparseRow<C::Scalar>,
    ) {
        self.a.0.push(a);
        self.b.0.push(b);
        self.c.0.push(c);
        self.m += 1;
    }

    pub(crate) fn public_wire(&mut self) -> Wire {
        let index = self.x.len();
        Wire::Instance(index)
    }

    pub(crate) fn private_wire(&mut self) -> Wire {
        let index = self.w.len();
        Wire::Witness(index)
    }

    /// constrain x * y = z
    pub fn mul_gate(
        &mut self,
        x: &SparseRow<C::Scalar>,
        y: &SparseRow<C::Scalar>,
        z: &SparseRow<C::Scalar>,
    ) {
        self.append(x.clone(), y.clone(), z.clone());
    }

    /// constrain x + y = z
    pub fn add_gate(
        &mut self,
        x: &SparseRow<C::Scalar>,
        y: &SparseRow<C::Scalar>,
        z: &SparseRow<C::Scalar>,
    ) {
        self.append(x + y, SparseRow::from(Wire::ONE), z.clone());
    }

    /// constrain x - y = z
    pub fn sub_gate(
        &mut self,
        x: &SparseRow<C::Scalar>,
        y: &SparseRow<C::Scalar>,
        z: &SparseRow<C::Scalar>,
    ) {
        self.append(x - y, SparseRow::from(Wire::ONE), z.clone());
    }

    /// constrain x == y
    pub fn equal_gate(&mut self, x: &SparseRow<C::Scalar>, y: &SparseRow<C::Scalar>) {
        self.mul_gate(x, &SparseRow::one(), y);
    }

    #[allow(clippy::type_complexity)]
    pub fn evaluate(&self) -> (Vec<C::Scalar>, Vec<C::Scalar>, Vec<C::Scalar>) {
        let a_evals = self.a.evaluate_with_z(&self.x, &self.w);
        let b_evals = self.b.evaluate_with_z(&self.x, &self.w);
        let c_evals = self.c.evaluate_with_z(&self.x, &self.w);
        (a_evals, b_evals, c_evals)
    }

    #[allow(clippy::type_complexity)]
    pub fn z_vectors(
        &self,
        l: usize,
        m_l_1: usize,
    ) -> (
        (
            Vec<Vec<(C::Scalar, usize)>>,
            Vec<Vec<(C::Scalar, usize)>>,
            Vec<Vec<(C::Scalar, usize)>>,
        ),
        (
            Vec<Vec<(C::Scalar, usize)>>,
            Vec<Vec<(C::Scalar, usize)>>,
            Vec<Vec<(C::Scalar, usize)>>,
        ),
    ) {
        let (a_x, a_w) = self.a.x_and_w(l, m_l_1);
        let (b_x, b_w) = self.b.x_and_w(l, m_l_1);
        let (c_x, c_w) = self.c.x_and_w(l, m_l_1);

        ((a_x, b_x, c_x), (a_w, b_w, c_w))
    }
}

impl<C: CircuitDriver> Default for R1cs<C> {
    fn default() -> Self {
        Self {
            m: 0,
            a: SparseMatrix::default(),
            b: SparseMatrix::default(),
            c: SparseMatrix::default(),
            x: DenseVectors::new(vec![C::Scalar::one()]),
            w: DenseVectors::default(),
        }
    }
}

impl<C: CircuitDriver> Index<Wire> for R1cs<C> {
    type Output = C::Scalar;

    fn index(&self, w: Wire) -> &Self::Output {
        match w {
            Wire::Instance(i) => &self.x[i],
            Wire::Witness(i) => &self.w[i],
        }
    }
}
