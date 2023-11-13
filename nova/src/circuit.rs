use r1cs::{CircuitDriver, R1cs};

pub struct NovaCircuit<C: CircuitDriver> {
    cs: R1cs<C>,
}

impl<C: CircuitDriver> Default for NovaCircuit<C> {
    fn default() -> Self {
        Self {
            cs: R1cs::default(),
        }
    }
}
