use core::ops::{Index, IndexMut};
use zkstd::common::{vec, Add, Mul, PrimeField, Sub, Vec};

#[derive(Clone, Debug, Default)]
pub struct DenseVectors<F: PrimeField>(Vec<F>);

impl<F: PrimeField> DenseVectors<F> {
    pub fn new(vectors: Vec<F>) -> Self {
        Self(vectors)
    }

    pub fn get(&self) -> Vec<F> {
        self.0.clone()
    }

    pub fn push(&mut self, vector: F) {
        self.0.push(vector)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> DenseVectorsIterator<F> {
        DenseVectorsIterator {
            dense_vectors: self.clone(),
            index: 0,
        }
    }

    pub fn one(m: usize) -> Self {
        Self(vec![F::one(); m])
    }

    pub fn zero(m: usize) -> Self {
        Self(vec![F::zero(); m])
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
