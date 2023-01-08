use zero_jubjub::{Fp as JubJubScalar, JubJubExtended, GENERATOR_EXTENDED};
use zero_plonk::prelude::*;

pub struct DummyCircuit {
    a: JubJubScalar,
    b: JubJubExtended,
}

impl DummyCircuit {
    pub fn new(a: JubJubScalar) -> Self {
        Self {
            a,
            b: GENERATOR_EXTENDED * &a,
        }
    }
}

impl Default for DummyCircuit {
    fn default() -> Self {
        Self::new(JubJubScalar::from(7u64))
    }
}

impl Circuit for DummyCircuit {
    fn circuit<C>(&self, composer: &mut C) -> Result<(), Error>
    where
        C: Composer,
    {
        let w_a = composer.append_witness(self.a);
        let w_b = composer.append_point(self.b);

        let w_x = composer.component_mul_generator(w_a, GENERATOR_EXTENDED)?;

        composer.assert_equal_point(w_b, w_x);

        Ok(())
    }
}

#[cfg(test)]
mod confidential_transfer_circuit_test {
    use super::*;
    use rand::{rngs::StdRng, SeedableRng};
    use zero_crypto::behave::Group;

    #[test]
    fn confidential_transfer_circuit_test() {
        let rng = &mut StdRng::seed_from_u64(8349u64);

        let n = 1 << 9;
        let label = b"demo";
        let pp = PublicParameters::setup(n, rng).expect("failed to create pp");

        let (prover, verifier) =
            Compiler::compile::<DummyCircuit>(&pp, label).expect("failed to compile circuit");

        // default works
        {
            let a = JubJubScalar::random(rng.clone());
            let (proof, public_inputs) = prover
                .prove(rng, &DummyCircuit::new(a))
                .expect("failed to prove");

            verifier
                .verify(&proof, &public_inputs)
                .expect("failed to verify proof");
        }
    }
}
