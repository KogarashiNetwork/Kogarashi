use core::ops::{Index, IndexMut};
use zkstd::common::{Add, Mul, PrimeField, Sub};

#[derive(Clone, Debug, Default)]
pub struct DenseVectors<F: PrimeField>(pub Vec<F>);

impl<F: PrimeField> DenseVectors<F> {
    pub fn iter(&self) -> DenseVectorsIterator<F> {
        DenseVectorsIterator {
            dense_vectors: self.clone(),
            index: 0,
        }
    }

    pub(crate) fn identity(m: usize) -> Self {
        Self(vec![F::one(); m])
    }
}

pub struct DenseVectorsIterator<F: PrimeField> {
    dense_vectors: DenseVectors<F>,
    index: usize,
}

impl<F: PrimeField> Iterator for DenseVectorsIterator<F> {
    type Item = F;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.dense_vectors.0.len() {
            let item = Some(self.dense_vectors[self.index]);
            self.index += 1;
            item
        } else {
            None
        }
    }
}

impl<F: PrimeField> Index<usize> for DenseVectors<F> {
    type Output = F;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<F: PrimeField> IndexMut<usize> for DenseVectors<F> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<F: PrimeField> Mul<F> for DenseVectors<F> {
    type Output = Self;

    fn mul(self, rhs: F) -> Self {
        Self(self.iter().map(|element| element * rhs).collect())
    }
}

/// Hadamard product
impl<F: PrimeField> Mul for DenseVectors<F> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.0.len(), rhs.0.len());

        Self(self.iter().zip(rhs.iter()).map(|(a, b)| a * b).collect())
    }
}

impl<F: PrimeField> Add for DenseVectors<F> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.0.len(), rhs.0.len());

        Self(self.iter().zip(rhs.iter()).map(|(a, b)| a + b).collect())
    }
}

impl<F: PrimeField> Sub for DenseVectors<F> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.0.len(), rhs.0.len());

        Self(self.iter().zip(rhs.iter()).map(|(a, b)| a - b).collect())
    }
}
