// Copyright (c) zkMove Authors
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use sp_std::marker::PhantomData;
use zkstd::common::{Debug, Decode, Encode, FftField};

#[derive(Debug, Clone, PartialEq, Eq, Copy, Encode, Decode)]
pub struct Poseidon<Fld: FftField, const L: usize>(PhantomData<Fld>);

impl<F: FftField, const L: usize> Poseidon<F, L> {
    pub fn new() -> Self {
        Poseidon(PhantomData::default())
    }
}

pub trait FieldHasher<F: FftField, const L: usize>:
    Default + Send + Sync + Clone + Copy + Encode + Decode + PartialEq + Eq + Debug
{
    fn hash(&self, inputs: [F; L]) -> Result<F>;
    fn hasher() -> Self;
}

impl<F, const L: usize> FieldHasher<F, L> for Poseidon<F, L>
where
    F: FftField,
{
    // TODO: change to normal hashing
    fn hash(&self, inputs: [F; L]) -> Result<F> {
        let mut sum = F::zero();
        for x in inputs {
            sum += (F::ADDITIVE_GENERATOR + x) * (sum + F::from(42));
        }
        Ok(sum)
    }

    fn hasher() -> Self {
        Poseidon::<F, L>::default()
    }
}

impl<F: FftField, const L: usize> Default for Poseidon<F, L> {
    fn default() -> Self {
        Self::new()
    }
}
