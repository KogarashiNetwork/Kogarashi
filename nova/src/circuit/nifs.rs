use core::marker::PhantomData;

use zkstd::circuit::prelude::{CircuitDriver, R1cs};

pub(crate) struct NifsCircuit<C: CircuitDriver> {
    p: PhantomData<C>,
}

impl<C: CircuitDriver> NifsCircuit<C> {
    pub(crate) fn verify(cs: &mut R1cs<C>) {}
}