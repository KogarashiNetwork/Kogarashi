use r1cs::{CircuitDriver, DenseVectors, R1cs, SparseMatrix};
use zkstd::common::{IntGroup, Ring};

pub(crate) struct RelaxedR1cs<C: CircuitDriver> {
    // 1. Structure S
    // a, b and c matrices and matrix size
    m: usize,
    a: SparseMatrix<C::Base>,
    b: SparseMatrix<C::Base>,
    c: SparseMatrix<C::Base>,

    // 2. Instance
    // r1cs instance includes public inputs and outputs, and error vector, scalar
    e: DenseVectors<C::Base>,
    u: C::Base,
    x: DenseVectors<C::Base>,

    // 3. Witness
    // r1cs witness includes private inputs and intermediate value
    w: DenseVectors<C::Base>,
}

impl<C: CircuitDriver> RelaxedR1cs<C> {
    pub(crate) fn new(r1cs: R1cs<C>) -> Self {
        let m = r1cs.m();
        let (a, b, c) = r1cs.matrices();
        let e = DenseVectors::new(vec![C::Base::zero(); m]);
        let u = C::Base::one();
        let x = DenseVectors::new(r1cs.x());
        let w = DenseVectors::new(r1cs.w());

        Self {
            m,
            a,
            b,
            c,
            e,
            u,
            x,
            w,
        }
    }

    ///  check (A · Z) ◦ (B · Z) = u · (C · Z) + E
    pub(crate) fn is_sat(&self) -> bool {
        let Self {
            m,
            a,
            b,
            c,
            e,
            u,
            x,
            w,
        } = self;
        // A · Z
        let az = a.prod(m, x, w);
        // B · Z
        let bz = b.prod(m, x, w);
        // C · Z
        let cz = c.prod(m, x, w);
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
    use super::RelaxedR1cs;
    use r1cs::{test::example_r1cs, GrumpkinDriver, R1cs};

    #[test]
    fn relaxed_r1cs_test() {
        for i in 1..10 {
            let r1cs: R1cs<GrumpkinDriver> = example_r1cs(i);
            let relaxed_r1cs = RelaxedR1cs::new(r1cs);
            assert!(relaxed_r1cs.is_sat())
        }
    }
}
