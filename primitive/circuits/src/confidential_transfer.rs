use zero_crypto::common::Group;
use zero_jubjub::{Fp as JubJubScalar, JubJubAffine, GENERATOR_EXTENDED};
use zero_plonk::prelude::*;

const BALANCE_BITS: usize = 16;

/// Confidential transfer circuit
pub struct ConfidentialTransferCircuit {
    alice_public_key: JubJubAffine,
    bob_public_key: JubJubAffine,
    alice_left_encrypted_balance: JubJubAffine,
    alice_right_encrypted_balance: JubJubAffine,
    alice_left_encrypted_transfer_amount: JubJubAffine,
    alice_right_encrypted_transfer_amount: JubJubAffine,
    bob_left_encrypted_transfer_amount: JubJubAffine,
    alice_private_key: JubJubScalar,
    transfer_amount_b: JubJubScalar,
    alice_after_balance: JubJubScalar,
    randomness: JubJubScalar,
    bits: usize,
}

impl ConfidentialTransferCircuit {
    /// Init confidential tranfer circuit
    pub fn new(
        alice_public_key: JubJubAffine,
        bob_public_key: JubJubAffine,
        alice_left_encrypted_balance: JubJubAffine,
        alice_right_encrypted_balance: JubJubAffine,
        alice_left_encrypted_transfer_amount: JubJubAffine,
        alice_right_encrypted_transfer_amount: JubJubAffine,
        bob_left_encrypted_transfer_amount: JubJubAffine,
        alice_private_key: JubJubScalar,
        transfer_amount_b: JubJubScalar,
        alice_after_balance: JubJubScalar,
        randomness: JubJubScalar,
    ) -> Self {
        Self {
            alice_public_key,
            bob_public_key,
            alice_left_encrypted_balance,
            alice_right_encrypted_balance,
            alice_left_encrypted_transfer_amount,
            alice_right_encrypted_transfer_amount,
            bob_left_encrypted_transfer_amount,
            alice_private_key,
            transfer_amount_b,
            alice_after_balance,
            randomness,
            bits: BALANCE_BITS,
        }
    }
}

impl Default for ConfidentialTransferCircuit {
    fn default() -> Self {
        Self {
            alice_public_key: JubJubAffine::identity(),
            bob_public_key: JubJubAffine::identity(),
            alice_left_encrypted_balance: JubJubAffine::identity(),
            alice_right_encrypted_balance: JubJubAffine::identity(),
            alice_left_encrypted_transfer_amount: JubJubAffine::identity(),
            alice_right_encrypted_transfer_amount: JubJubAffine::identity(),
            bob_left_encrypted_transfer_amount: JubJubAffine::identity(),
            alice_private_key: JubJubScalar::ADDITIVE_IDENTITY,
            transfer_amount_b: JubJubScalar::ADDITIVE_IDENTITY,
            alice_after_balance: JubJubScalar::ADDITIVE_IDENTITY,
            randomness: JubJubScalar::ADDITIVE_IDENTITY,
            bits: BALANCE_BITS,
        }
    }
}

impl Circuit for ConfidentialTransferCircuit {
    fn circuit<C>(&self, composer: &mut C) -> Result<(), Error>
    where
        C: Composer,
    {
        let alice_public_key = composer.append_point(self.alice_public_key);
        let bob_public_key = composer.append_point(self.bob_public_key);
        let alice_left_encrypted_balance = composer.append_point(self.alice_left_encrypted_balance);
        let alice_right_encrypted_balance =
            composer.append_point(self.alice_right_encrypted_balance);
        let alice_left_encrypted_transfer_amount =
            composer.append_point(self.alice_left_encrypted_transfer_amount);
        let alice_right_encrypted_transfer_amount =
            composer.append_point(self.alice_right_encrypted_transfer_amount);
        let alice_private_key = composer.append_witness(self.alice_private_key);
        let transfer_amount_b = composer.append_witness(self.transfer_amount_b);
        let alice_after_balance = composer.append_witness(self.alice_after_balance);
        let randomness = composer.append_witness(self.randomness);
        let neg = composer.append_witness(-JubJubScalar::one());

        // Alice left encrypted transfer check
        let g_pow_balance =
            composer.component_mul_generator(transfer_amount_b, GENERATOR_EXTENDED)?;
        let alice_pk_powered_by_randomness =
            composer.component_mul_point(randomness, alice_public_key);
        let left_alice_transfer =
            composer.component_add_point(g_pow_balance, alice_pk_powered_by_randomness);
        composer.assert_equal_public_point(
            left_alice_transfer,
            self.alice_left_encrypted_transfer_amount,
        );

        // Bob left encrypted transfer check
        let bob_pk_powered_by_randomness = composer.component_mul_point(randomness, bob_public_key);
        let left_bob_transfer =
            composer.component_add_point(g_pow_balance, bob_pk_powered_by_randomness);
        composer
            .assert_equal_public_point(left_bob_transfer, self.bob_left_encrypted_transfer_amount);

        // Alice right encrypted transfer check
        let g_pow_randomness = composer.component_mul_generator(randomness, GENERATOR_EXTENDED)?;
        composer.assert_equal_public_point(
            g_pow_randomness,
            self.alice_right_encrypted_transfer_amount,
        );

        // Alice after balance check
        let g_pow_after_balance =
            composer.component_mul_generator(alice_after_balance, GENERATOR_EXTENDED)?;
        let alice_left_transfer_neg =
            composer.component_mul_point(neg, alice_left_encrypted_transfer_amount);
        let alice_right_transfer_neg =
            composer.component_mul_point(neg, alice_right_encrypted_transfer_amount);
        let left_after_balance =
            composer.component_add_point(alice_left_encrypted_balance, alice_left_transfer_neg);
        let right_after_balance = {
            let right_after_balance = composer
                .component_add_point(alice_right_encrypted_balance, alice_right_transfer_neg);
            composer.component_mul_point(alice_private_key, right_after_balance)
        };
        let x = composer.component_add_point(g_pow_after_balance, right_after_balance);
        composer.assert_equal_point(left_after_balance, x);

        // Public key calculation check
        let calculated_pk =
            composer.component_mul_generator(alice_private_key, GENERATOR_EXTENDED)?;
        composer.assert_equal_public_point(calculated_pk, self.alice_public_key);

        // Transfer amount and ramaining balance range check
        composer.component_range(transfer_amount_b, self.bits);
        composer.component_range(alice_after_balance, self.bits);

        Ok(())
    }
}

#[cfg(test)]
mod confidential_transfer_circuit_test {
    use super::*;
    use ark_std::{end_timer, start_timer};
    use rand::{rngs::StdRng, SeedableRng};
    use zero_crypto::behave::Group;

    #[test]
    fn confidential_transfer_circuit_test() {
        // 1. trusted setup and key pair generation
        let mut rng = StdRng::seed_from_u64(8349u64);
        let n = 1 << 14;
        let label = b"demo";
        let trusted_setup = start_timer!(|| "trusted setup");
        let pp = PublicParameters::setup(n, &mut rng).expect("failed to create pp");
        end_timer!(trusted_setup);

        let circuit_compile = start_timer!(|| "circuit compile");
        let (prover, verifier) = Compiler::compile::<ConfidentialTransferCircuit>(&pp, label)
            .expect("failed to compile circuit");
        end_timer!(circuit_compile);

        // 2. confidential transfer params
        // 2.0. transaction sender and recipient key pair
        let params_generation = start_timer!(|| "params generation");
        let generator = GENERATOR_EXTENDED;
        let alice_private_key = JubJubScalar::random(&mut rng);
        let bob_private_key = JubJubScalar::random(&mut rng);
        let alice_public_key = generator * alice_private_key;
        let bob_public_key = generator * bob_private_key;

        // 2.1. encrypt transaction by ElGamal encryption
        let alice_balance = JubJubScalar::from(1500 as u64);
        let transfer_amount_b = JubJubScalar::from(800 as u64);
        let alice_after_balance = JubJubScalar::from(700 as u64);
        let alice_original_randomness = JubJubScalar::from(789 as u64);
        let randomness = JubJubScalar::from(123 as u64);

        let alice_left_encrypted_balance =
            (generator * alice_balance) + (alice_public_key * alice_original_randomness);
        let alice_right_encrypted_balance = generator * alice_original_randomness;
        let alice_left_encrypted_transfer_amount =
            (generator * transfer_amount_b) + (alice_public_key * randomness);
        let alice_right_encrypted_transfer_amount = generator * randomness;
        let bob_left_encrypted_transfer_amount =
            (generator * transfer_amount_b) + (bob_public_key * randomness);
        end_timer!(params_generation);

        // 3. generate proof
        let proof_generation = start_timer!(|| "proof generation");
        let (proof, public_inputs) = prover
            .prove(
                &mut rng,
                &ConfidentialTransferCircuit::new(
                    JubJubAffine::from(alice_public_key),
                    JubJubAffine::from(bob_public_key),
                    JubJubAffine::from(alice_left_encrypted_balance),
                    JubJubAffine::from(alice_right_encrypted_balance),
                    JubJubAffine::from(alice_left_encrypted_transfer_amount),
                    JubJubAffine::from(alice_right_encrypted_transfer_amount),
                    JubJubAffine::from(bob_left_encrypted_transfer_amount),
                    alice_private_key,
                    transfer_amount_b,
                    alice_after_balance,
                    randomness,
                ),
            )
            .expect("failed to prove");
        end_timer!(proof_generation);

        // 4. verify proof
        let verify_proof = start_timer!(|| "verify proof");
        verifier
            .verify(&proof, &public_inputs)
            .expect("failed to verify proof");
        end_timer!(verify_proof);
    }
}
