use crate::{PedersenCommitment, R1csShape};
use zkstd::circuit::prelude::CircuitDriver;
use zkstd::common::{IntGroup, PrimeField};
use zkstd::matrix::DenseVectors;

/// A type that holds a witness for a given R1CS instance
#[derive(Clone, Debug)]
pub struct R1csWitness<C: CircuitDriver> {
    pub w: DenseVectors<C::Scalar>,
}

impl<C: CircuitDriver> R1csWitness<C> {
    pub fn new(shape: &R1csShape<C>, w: Vec<C::Scalar>) -> Self {
        assert_eq!(shape.m_l_1(), w.len());
        Self {
            w: DenseVectors::new(w),
        }
    }

    /// Commits to the witness using the supplied generators
    pub fn commit(&self, ck: &PedersenCommitment<C::Affine>) -> C::Affine {
        ck.commit(&self.w)
    }
}

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

    pub fn from_r1cs_witness(shape: &R1csShape<C>, witness: &R1csWitness<C>) -> Self {
        Self {
            w: witness.w.clone(),
            e: DenseVectors::new(vec![C::Scalar::zero(); shape.m()]),
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
