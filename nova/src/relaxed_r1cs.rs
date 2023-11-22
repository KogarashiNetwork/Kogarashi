mod instance;
mod witness;

use crate::transcript::Transcript;
pub(crate) use instance::RelaxedR1csInstance;
use r1cs::{CircuitDriver, DenseVectors, R1cs, SparseMatrix};
pub(crate) use witness::RelaxedR1csWitness;

#[derive(Clone, Debug)]
pub struct RelaxedR1cs<C: CircuitDriver> {
    // 1. Structure S
    // a, b and c matrices and matrix size
    m: usize,
    a: SparseMatrix<C::Scalar>,
    b: SparseMatrix<C::Scalar>,
    c: SparseMatrix<C::Scalar>,

    // 2. Instance
    // r1cs instance includes public inputs, outputs and scalar
    pub(crate) instance: RelaxedR1csInstance<C>,

    // 3. Witness
    // r1cs witness includes private inputs, intermediate value and error vector
    pub(crate) witness: RelaxedR1csWitness<C>,
}

impl<C: CircuitDriver> RelaxedR1cs<C> {
    pub fn new(r1cs: R1cs<C>) -> Self {
        let m = r1cs.m();
        let (a, b, c) = r1cs.matrices();
        let x = DenseVectors::new(r1cs.x());
        let w = DenseVectors::new(r1cs.w());

        let instance = RelaxedR1csInstance::default(x);
        let witness = RelaxedR1csWitness::default(w);

        Self {
            m,
            a,
            b,
            c,
            instance,
            witness,
        }
    }

    pub(crate) fn u(&self) -> C::Scalar {
        self.instance.u
    }

    pub(crate) fn x(&self) -> DenseVectors<C::Scalar> {
        self.instance.x.clone()
    }

    pub(crate) fn w(&self) -> DenseVectors<C::Scalar> {
        self.witness.w.clone()
    }

    pub(crate) fn fold_instance(
        &self,
        r1cs: &R1cs<C>,
        r: C::Scalar,
        commit_t: C::Affine,
    ) -> RelaxedR1csInstance<C> {
        self.instance.fold(r1cs, r, commit_t)
    }

    pub(crate) fn fold_witness(
        &self,
        r1cs: &R1cs<C>,
        r: C::Scalar,
        t: DenseVectors<C::Scalar>,
    ) -> RelaxedR1csWitness<C> {
        self.witness.fold(r1cs, r, t)
    }

    pub(crate) fn update(
        &self,
        instance: &RelaxedR1csInstance<C>,
        witness: &RelaxedR1csWitness<C>,
    ) -> Self {
        let RelaxedR1cs {
            m,
            a,
            b,
            c,
            instance: _,
            witness: _,
        } = self.clone();
        Self {
            m,
            a,
            b,
            c,
            instance: instance.clone(),
            witness: witness.clone(),
        }
    }

    ///  check (A · Z) ◦ (B · Z) = u · (C · Z) + E
    pub fn is_sat(&self) -> bool {
        let Self {
            m,
            a,
            b,
            c,
            instance,
            witness,
        } = self;
        let RelaxedR1csInstance {
            commit_w: _,
            commit_e: _,
            u,
            x,
        } = instance;
        let RelaxedR1csWitness { w, e } = witness;

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

    pub fn absorb_by_transcript<T: Transcript<C>>(&self, transcript: &mut T) {
        self.instance.absorb_by_transcript(transcript);
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
