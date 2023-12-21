use crate::{PedersenCommitment, R1csShape};
use zkstd::circuit::prelude::CircuitDriver;
use zkstd::common::{Decode, Encode, IntGroup};
use zkstd::matrix::DenseVectors;

/// A type that holds a witness for a given R1CS instance
#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
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

    pub fn commit(&self, ck: &PedersenCommitment<C::Affine>) -> C::Affine {
        ck.commit(&self.w)
    }

    pub(crate) fn w(&self) -> DenseVectors<C::Scalar> {
        self.w.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
pub struct RelaxedR1csWitness<C: CircuitDriver> {
    /// witness
    pub(crate) w: DenseVectors<C::Scalar>,
    /// error vectors
    pub(crate) e: DenseVectors<C::Scalar>,
}

impl<C: CircuitDriver> RelaxedR1csWitness<C> {
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
        witness: &R1csWitness<C>,
        r: C::Scalar,
        t: DenseVectors<C::Scalar>,
    ) -> Self {
        let w1 = self.w();
        let w2 = witness.w();
        let e1 = self.e.clone();

        let e = e1 + t * r;
        let w = w1 + w2 * r;

        Self { e, w }
    }
}
