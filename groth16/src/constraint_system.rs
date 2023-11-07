use core::ops::Index;
use r1cs::{DenseVectors, R1cs, SparseRow, Wire};
use zkstd::common::{Ring, TwistedEdwardsAffine};

#[derive(Debug)]
pub struct ConstraintSystem<C: TwistedEdwardsAffine> {
    pub(crate) constraints: R1cs<C::Range>,
    pub(crate) x: DenseVectors<C::Range>,
    pub(crate) w: DenseVectors<C::Range>,
}

impl<C: TwistedEdwardsAffine> Index<Wire> for ConstraintSystem<C> {
    type Output = C::Range;

    fn index(&self, w: Wire) -> &Self::Output {
        match w {
            Wire::Instance(i) => &self.x[i],
            Wire::Witness(i) => &self.w[i],
        }
    }
}

impl<C: TwistedEdwardsAffine> ConstraintSystem<C> {
    pub(crate) fn initialize() -> Self {
        Self {
            constraints: R1cs::default(),
            x: DenseVectors::new([C::Range::one()].to_vec()),
            w: DenseVectors::default(),
        }
    }

    pub(crate) fn m(&self) -> usize {
        self.constraints.m()
    }

    fn alloc_instance(&mut self, instance: C::Range) -> Wire {
        let wire = self.public_wire();
        self.x.push(instance);
        wire
    }

    fn alloc_witness(&mut self, witness: C::Range) -> Wire {
        let wire = self.private_wire();
        self.w.push(witness);
        wire
    }

    pub(crate) fn instance_len(&self) -> usize {
        self.x.len()
    }

    pub(crate) fn witness_len(&self) -> usize {
        self.w.len()
    }

    /// Add a public wire to the gadget. It will start with no generator and no associated constraints.
    pub fn public_wire(&mut self) -> Wire {
        let index = self.x.len();
        Wire::Instance(index)
    }

    /// Add a private wire to the gadget. It will start with no generator and no associated constraints.
    fn private_wire(&mut self) -> Wire {
        let index = self.w.len();
        Wire::Witness(index)
    }

    /// Assert that x * y = z;
    pub fn assert_product(
        &mut self,
        x: &SparseRow<C::Range>,
        y: &SparseRow<C::Range>,
        z: &SparseRow<C::Range>,
    ) {
        self.constraints.append(x.clone(), y.clone(), z.clone());
    }

    // Assert that x + y = z;
    pub fn assert_sum(
        &mut self,
        x: &SparseRow<C::Range>,
        y: &SparseRow<C::Range>,
        z: &SparseRow<C::Range>,
    ) {
        self.constraints
            .append(x + y, SparseRow::from(Wire::ONE), z.clone());
    }

    /// Assert that x == y.
    pub fn assert_equal(&mut self, x: &SparseRow<C::Range>, y: &SparseRow<C::Range>) {
        self.assert_product(x, &SparseRow::one(), y);
    }

    /// The product of two `SparseRow`s `x` and `y`, i.e. `x * y`.
    pub fn product(
        &mut self,
        x: &SparseRow<C::Range>,
        y: &SparseRow<C::Range>,
    ) -> SparseRow<C::Range> {
        if let Some(c) = x.as_constant() {
            return y * c;
        }
        if let Some(c) = y.as_constant() {
            return x * c;
        }

        let product_value = x.evaluate(&self.x, &self.w) * y.evaluate(&self.x, &self.w);
        let product = self.alloc_witness(product_value);
        let product_exp = SparseRow::from(product);
        self.assert_product(x, y, &product_exp);

        product_exp
    }

    /// The product of two `SparseRow`s `x` and `y`, i.e. `x * y`.
    pub fn sum(&mut self, x: &SparseRow<C::Range>, y: &SparseRow<C::Range>) -> SparseRow<C::Range> {
        if let Some(c) = x.as_constant() {
            return y * c;
        }
        if let Some(c) = y.as_constant() {
            return x * c;
        }

        let sum_value = x.evaluate(&self.x, &self.w) + y.evaluate(&self.x, &self.w);
        let sum = self.alloc_witness(sum_value);
        let sum_exp = SparseRow::from(sum);
        self.assert_sum(x, y, &sum_exp);
        sum_exp
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::circuit::Circuit;
    use crate::constraint_system::ConstraintSystem;
    use crate::error::Error;
    use crate::zksnark::ZkSnark;
    use bls_12_381::Fr as BlsScalar;
    use ec_pairing::TatePairing;
    use jub_jub::JubjubAffine;
    use zkstd::common::OsRng;

    #[test]
    fn arithmetic_test() {
        #[derive(Debug)]
        pub struct DummyCircuit {
            x: BlsScalar,
            o: BlsScalar,
        }

        impl DummyCircuit {
            pub fn new(x: BlsScalar, o: BlsScalar) -> Self {
                Self { x, o }
            }
        }

        impl Default for DummyCircuit {
            fn default() -> Self {
                Self::new(0.into(), 0.into())
            }
        }

        impl Circuit<JubjubAffine> for DummyCircuit {
            fn synthesize(
                &self,
                composer: &mut ConstraintSystem<JubjubAffine>,
            ) -> Result<(), Error> {
                let x = SparseRow::from(composer.alloc_instance(self.x));
                let o = composer.alloc_instance(self.o);

                let sym1 = composer.product(&x, &x);
                let y = composer.product(&sym1, &x);
                let sym2 = composer.sum(&y, &x);

                composer.assert_equal(
                    &(sym2 + SparseRow::from(BlsScalar::from(5))),
                    &SparseRow::from(o),
                );

                Ok(())
            }
        }

        let x = BlsScalar::from(3);
        let o = BlsScalar::from(35);
        let circuit = DummyCircuit::new(x, o);

        let (mut prover, verifier) =
            ZkSnark::<TatePairing, JubjubAffine>::setup::<DummyCircuit>(OsRng)
                .expect("Failed to compile circuit");
        let proof = prover
            .create_proof(&mut OsRng, circuit)
            .expect("Failed to prove");
        verifier
            .verify(&proof, &[x, o])
            .expect("Failed to verify the proof");
    }
}
