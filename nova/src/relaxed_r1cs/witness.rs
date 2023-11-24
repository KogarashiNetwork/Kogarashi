use zkstd::circuit::prelude::{CircuitDriver, R1cs};
use zkstd::common::{IntGroup, PrimeField};
use zkstd::matrix::DenseVectors;

#[derive(Clone, Debug)]
pub struct RelaxedR1csWitness<C: CircuitDriver> {
    /// witness
    pub(crate) w: DenseVectors<C::Scalar>,
    /// error vectors
    pub(crate) e: DenseVectors<C::Scalar>,
}

impl<C: CircuitDriver> RelaxedR1csWitness<C> {
    pub(crate) fn default(w: DenseVectors<C::Scalar>) -> Self {
        Self {
            e: DenseVectors::new(vec![C::Scalar::zero(); w.get().len()]),
            w,
        }
    }

    pub(crate) fn fold(&self, r1cs: &R1cs<C>, r: C::Scalar, t: DenseVectors<C::Scalar>) -> Self {
        let r2 = r.square();
        let e2 = self.e.clone();
        let w1 = DenseVectors::new(r1cs.w());
        let w2 = self.w.clone();

        let e = t * r + e2 * r2;
        let w = w1 + w2 * r;

        Self { e, w }
    }
}
