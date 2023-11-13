use crate::{NovaCircuit, Prover, RelaxedR1cs};

use r1cs::{CircuitDriver, DenseVectors, R1cs};
use zkstd::common::RngCore;

pub struct Ivc<C: CircuitDriver> {
    i: usize,
    cs: NovaCircuit<C>,
    z0: DenseVectors<C::Scalar>,
    zi: DenseVectors<C::Scalar>,
    prover: Prover<C>,
    r1cs: R1cs<C>,
    relaxed_r1cs: RelaxedR1cs<C>,
}

impl<C: CircuitDriver> Ivc<C> {
    pub fn new(r1cs: R1cs<C>, rng: impl RngCore, z0: DenseVectors<C::Scalar>) -> Self {
        let i = 0;
        let cs = NovaCircuit::default();
        let zi = z0.clone();
        let prover = Prover::new(r1cs.clone(), rng);
        let relaxed_r1cs = RelaxedR1cs::new(r1cs.clone());

        Self {
            i,
            cs,
            z0,
            zi,
            prover,
            r1cs,
            relaxed_r1cs,
        }
    }
}
