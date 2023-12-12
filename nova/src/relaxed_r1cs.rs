mod instance;
mod witness;

pub(crate) use instance::RelaxedR1csInstance;
pub(crate) use witness::RelaxedR1csWitness;
use zkstd::circuit::prelude::{CircuitDriver, R1cs};
use zkstd::matrix::{DenseVectors, SparseMatrix};

#[derive(Clone, Debug)]
pub struct R1csShape<C: CircuitDriver> {
    // 1. Structure S
    // a, b and c matrices and matrix size
    m: usize,
    instance_length: usize,
    witness_length: usize,
    a: SparseMatrix<C::Scalar>,
    b: SparseMatrix<C::Scalar>,
    c: SparseMatrix<C::Scalar>,
}

impl<C: CircuitDriver> From<R1cs<C>> for R1csShape<C> {
    fn from(value: R1cs<C>) -> Self {
        let (a, b, c) = value.matrices();
        Self {
            m: value.m(),
            instance_length: value.l(),
            witness_length: value.m_l_1(),
            a,
            b,
            c,
        }
    }
}

impl<C: CircuitDriver> R1csShape<C> {
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

    pub fn m(&self) -> usize {
        self.m
    }

    pub fn l(&self) -> usize {
        self.instance_length
    }

    pub fn m_l_1(&self) -> usize {
        self.witness_length
    }

    ///  check (A · Z) ◦ (B · Z) = u · (C · Z) + E
    pub fn is_sat(
        &self,
        instance: &RelaxedR1csInstance<C>,
        witness: &RelaxedR1csWitness<C>,
    ) -> bool {
        let Self { m, a, b, c, .. } = self;

        let RelaxedR1csInstance {
            commit_w: _,
            commit_e: _,
            u,
            x,
        } = instance;
        let RelaxedR1csWitness { w, e } = witness;

        let l = x.len() + 1;
        let z = DenseVectors::new(vec![vec![*u], x.get(), w.get()].concat());
        // A · Z
        let az = a.prod(m, l, &z);
        // B · Z
        let bz = b.prod(m, l, &z);
        // C · Z
        let cz = c.prod(m, l, &z);
        // (A · Z) ◦ (B · Z)
        let azbz = az * bz;

        // u · (C · Z) + E
        let ucz = cz * *u;
        let ucze = ucz + e.clone();

        azbz.iter()
            .zip(ucze.iter())
            .all(|(left, right)| left == right)
    }
}

#[cfg(test)]
mod tests {
    use super::{R1csShape, RelaxedR1csInstance, RelaxedR1csWitness};

    use crate::driver::GrumpkinDriver;
    use zkstd::circuit::prelude::R1cs;
    use zkstd::matrix::DenseVectors;
    use zkstd::r1cs::test::example_r1cs;

    #[test]
    fn relaxed_r1cs_test() {
        for i in 1..10 {
            let r1cs: R1cs<GrumpkinDriver> = example_r1cs(i);
            let instance = RelaxedR1csInstance::new(DenseVectors::new(r1cs.x()));
            let witness = RelaxedR1csWitness::new(DenseVectors::new(r1cs.w()), r1cs.m());
            let relaxed_r1cs = R1csShape::from(r1cs);
            assert!(relaxed_r1cs.is_sat(&instance, &witness))
        }
    }
}
