use zkstd::circuit::prelude::CircuitDriver;
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
    pub(crate) fn new(w: DenseVectors<C::Scalar>, m: usize) -> Self {
        Self {
            e: DenseVectors::new(vec![C::Scalar::zero(); m]),
            w,
        }
    }

    pub(crate) fn w(&self) -> DenseVectors<C::Scalar> {
        self.w.clone()
    }

    pub(crate) fn dummy(w_len: usize, m: usize) -> Self {
        Self {
            e: DenseVectors::zero(m),
            w: DenseVectors::zero(w_len),
        }
    }

    pub(crate) fn fold(
        &self,
        witness: &RelaxedR1csWitness<C>,
        r: C::Scalar,
        t: DenseVectors<C::Scalar>,
    ) -> Self {
        let r2 = r.square();
        let e2 = self.e.clone();
        let w1 = witness.w();
        let w2 = self.w();

        let e = t * r + e2 * r2;
        let w = w1 + w2 * r;

        Self { e, w }
    }
}
