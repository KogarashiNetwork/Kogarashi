use zero_crypto::common::{Affine, Basic, Curve, Decode, Encode, Group};
use zero_elgamal::EncryptedNumber;
use zero_jubjub::{Fp as JubJubScalar, JubJubAffine, GENERATOR_EXTENDED};
use zero_plonk::prelude::*;

pub const BALANCE_BITS: usize = 16;
pub const CONFIDENTIAL_TRANSFER_PUBLIC_INPUT_LENGTH: usize = 8;

/// Confidential transfer circuit
pub struct ConfidentialTransferCircuit {
    sender_public_key: JubJubAffine,
    recipient_public_key: JubJubAffine,
    sender_encrypted_balance: EncryptedNumber,
    sender_encrypted_transfer_amount: EncryptedNumber,
    recipient_encrypted_transfer_amount: JubJubAffine,
    sender_private_key: JubJubScalar,
    transfer_amount: JubJubScalar,
    sender_after_balance: JubJubScalar,
    randomness: JubJubScalar,
    bits: usize,
}

impl ConfidentialTransferCircuit {
    /// Init confidential tranfer circuit
    pub fn new(
        sender_public_key: JubJubAffine,
        recipient_public_key: JubJubAffine,
        sender_encrypted_balance: EncryptedNumber,
        sender_encrypted_transfer_amount: EncryptedNumber,
        recipient_encrypted_transfer_amount: JubJubAffine,
        sender_private_key: JubJubScalar,
        transfer_amount: JubJubScalar,
        sender_after_balance: JubJubScalar,
        randomness: JubJubScalar,
    ) -> Self {
        Self {
            sender_public_key,
            recipient_public_key,
            sender_encrypted_balance,
            sender_encrypted_transfer_amount,
            recipient_encrypted_transfer_amount,
            sender_private_key,
            transfer_amount,
            sender_after_balance,
            randomness,
            bits: BALANCE_BITS,
        }
    }
}

impl Default for ConfidentialTransferCircuit {
    fn default() -> Self {
        Self {
            sender_public_key: JubJubAffine::identity(),
            recipient_public_key: JubJubAffine::identity(),
            sender_encrypted_balance: EncryptedNumber::default(),
            sender_encrypted_transfer_amount: EncryptedNumber::default(),
            recipient_encrypted_transfer_amount: JubJubAffine::identity(),
            sender_private_key: JubJubScalar::ADDITIVE_IDENTITY,
            transfer_amount: JubJubScalar::ADDITIVE_IDENTITY,
            sender_after_balance: JubJubScalar::ADDITIVE_IDENTITY,
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
        let (alice_t_balance, alice_s_balance) = self.sender_encrypted_balance.get();
        let (alice_t_transfer_amount, alice_s_transfer_amount) =
            self.sender_encrypted_transfer_amount.get();
        let sender_public_key = composer.append_point(self.sender_public_key);
        let recipient_public_key = composer.append_point(self.recipient_public_key);
        let alice_t_encrypted_balance = composer.append_point(alice_t_balance);
        let alice_s_encrypted_balance = composer.append_point(alice_s_balance);
        let alice_t_encrypted_transfer_amount = composer.append_point(alice_t_transfer_amount);
        let alice_s_encrypted_transfer_amount = composer.append_point(alice_s_transfer_amount);
        let sender_private_key = composer.append_witness(self.sender_private_key);
        let transfer_amount = composer.append_witness(self.transfer_amount);
        let sender_after_balance = composer.append_witness(self.sender_after_balance);
        let randomness = composer.append_witness(self.randomness);
        let neg = composer.append_witness(-JubJubScalar::one());

        // Alice left encrypted transfer check
        let g_pow_balance =
            composer.component_mul_generator(transfer_amount, GENERATOR_EXTENDED)?;
        let alice_pk_powered_by_randomness =
            composer.component_mul_point(randomness, sender_public_key);
        let s_alice_transfer =
            composer.component_add_point(g_pow_balance, alice_pk_powered_by_randomness);
        composer.assert_equal_public_point(s_alice_transfer, alice_t_transfer_amount);

        // Bob left encrypted transfer check
        let bob_pk_powered_by_randomness =
            composer.component_mul_point(randomness, recipient_public_key);
        let s_bob_transfer =
            composer.component_add_point(g_pow_balance, bob_pk_powered_by_randomness);
        composer
            .assert_equal_public_point(s_bob_transfer, self.recipient_encrypted_transfer_amount);

        // Alice right encrypted transfer check
        let g_pow_randomness = composer.component_mul_generator(randomness, GENERATOR_EXTENDED)?;
        composer.assert_equal_public_point(g_pow_randomness, alice_s_transfer_amount);

        // Alice after balance check
        let g_pow_after_balance =
            composer.component_mul_generator(sender_after_balance, GENERATOR_EXTENDED)?;
        let alice_t_transfer_neg =
            composer.component_mul_point(neg, alice_t_encrypted_transfer_amount);
        let alice_s_transfer_neg =
            composer.component_mul_point(neg, alice_s_encrypted_transfer_amount);
        let s_after_balance =
            composer.component_add_point(alice_t_encrypted_balance, alice_t_transfer_neg);
        let right_after_balance = {
            let right_after_balance =
                composer.component_add_point(alice_s_encrypted_balance, alice_s_transfer_neg);
            composer.component_mul_point(sender_private_key, right_after_balance)
        };
        let x = composer.component_add_point(g_pow_after_balance, right_after_balance);
        composer.assert_equal_point(s_after_balance, x);

        // Public key calculation check
        let calculated_pk =
            composer.component_mul_generator(sender_private_key, GENERATOR_EXTENDED)?;
        composer.assert_equal_public_point(calculated_pk, self.sender_public_key);

        // Transfer amount and ramaining balance range check
        composer.component_range(transfer_amount, self.bits);
        composer.component_range(sender_after_balance, self.bits);

        Ok(())
    }
}

pub trait Encrypted {
    type Affine: Basic;

    fn get_s_and_t(self) -> (Self::Affine, Self::Affine);
}

/// confidential transfer transaction input
#[derive(Clone, Debug, Decode, Encode, Eq, PartialEq)]
pub struct ConfidentialTransferTransaction<E: Encrypted> {
    /// sender public key
    pub sender_public_key: E::Affine,
    /// recipient public key
    pub recipient_public_key: E::Affine,
    /// encrypted transfer amount by sender
    pub sender_encrypted_transfer_amount: E,
    /// encrypted transfer amount by recipient
    pub recipient_encrypted_transfer_amount: E::Affine,
}

impl<E: Encrypted> ConfidentialTransferTransaction<E> {
    /// init confidential transfer transaction
    pub fn new(
        sender_public_key: E::Affine,
        recipient_public_key: E::Affine,
        sender_encrypted_transfer_amount: E,
        recipient_encrypted_transfer_amount: E::Affine,
    ) -> Self {
        Self {
            sender_public_key,
            recipient_public_key,
            sender_encrypted_transfer_amount,
            recipient_encrypted_transfer_amount,
        }
    }

    /// output public inputs for confidential transfer transaction
    pub fn public_inputs(
        self,
    ) -> [<<E as Encrypted>::Affine as Curve>::Range; CONFIDENTIAL_TRANSFER_PUBLIC_INPUT_LENGTH]
    {
        let mut public_inputs = [<<E as Encrypted>::Affine as Curve>::Range::zero();
            CONFIDENTIAL_TRANSFER_PUBLIC_INPUT_LENGTH];
        let (sender_s, sender_t) = self.sender_encrypted_transfer_amount.get_s_and_t();
        for (i, public_point) in [
            sender_t,
            self.recipient_encrypted_transfer_amount,
            sender_s,
            self.sender_public_key,
        ]
        .iter()
        .enumerate()
        {
            let (x, y) = (-public_point.get_x(), -public_point.get_y());
            public_inputs[i * 2] = x;
            public_inputs[i * 2] = y;
        }
        public_inputs
    }
}

#[cfg(test)]
mod confidential_transfer_circuit_test {
    use std::println;

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
        let transfer_amount = JubJubScalar::from(800 as u64);
        let alice_after_balance = JubJubScalar::from(700 as u64);
        let alice_original_randomness = JubJubScalar::from(789 as u64);
        let randomness = JubJubScalar::from(123 as u64);

        let alice_t_encrypted_balance =
            (generator * alice_balance) + (alice_public_key * alice_original_randomness);
        let alice_s_encrypted_balance = generator * alice_original_randomness;
        let alice_t_encrypted_transfer_amount =
            (generator * transfer_amount) + (alice_public_key * randomness);
        let alice_s_encrypted_transfer_amount = generator * randomness;
        let bob_encrypted_transfer_amount =
            (generator * transfer_amount) + (bob_public_key * randomness);
        let alice_public_key = JubJubAffine::from(alice_public_key);
        let bob_public_key = JubJubAffine::from(bob_public_key);
        let alice_t_encrypted_balance = JubJubAffine::from(alice_t_encrypted_balance);
        let alice_s_encrypted_balance = JubJubAffine::from(alice_s_encrypted_balance);
        let alice_t_encrypted_transfer_amount =
            JubJubAffine::from(alice_t_encrypted_transfer_amount);
        let alice_s_encrypted_transfer_amount =
            JubJubAffine::from(alice_s_encrypted_transfer_amount);
        let bob_encrypted_transfer_amount = JubJubAffine::from(bob_encrypted_transfer_amount);
        end_timer!(params_generation);

        // 2.2. init confidential transfer transaction
        let transaction = ConfidentialTransferTransaction::new(
            alice_t_encrypted_transfer_amount,
            bob_encrypted_transfer_amount,
            alice_s_encrypted_transfer_amount,
            alice_public_key,
        );
        let public_inputs = transaction.public_inputs();

        // 3. generate proof
        let proof_generation = start_timer!(|| "proof generation");
        let (proof, _) = prover
            .prove(
                &mut rng,
                &ConfidentialTransferCircuit::new(
                    alice_public_key,
                    bob_public_key,
                    EncryptedNumber::new(alice_t_encrypted_balance, alice_s_encrypted_balance),
                    EncryptedNumber::new(
                        alice_t_encrypted_transfer_amount,
                        alice_s_encrypted_transfer_amount,
                    ),
                    bob_encrypted_transfer_amount,
                    alice_private_key,
                    transfer_amount,
                    alice_after_balance,
                    randomness,
                ),
            )
            .expect("failed to prove");
        end_timer!(proof_generation);

        println!("\n\n{:?}", public_inputs);
        println!(
            "\n\nalice_t_encrypted_transfer_amount x: {:?}, y: {:?}",
            -alice_t_encrypted_transfer_amount.get_x(),
            -alice_t_encrypted_transfer_amount.get_y()
        );
        println!(
            "\n\nbob_encrypted_transfer_amount x: {:?}, y: {:?}",
            -bob_encrypted_transfer_amount.get_x(),
            -bob_encrypted_transfer_amount.get_y()
        );
        println!(
            "\n\nalice_s_encrypted_transfer_amount x: {:?}, y: {:?}",
            -alice_s_encrypted_transfer_amount.get_x(),
            -alice_s_encrypted_transfer_amount.get_y()
        );
        println!(
            "\n\nalice_public_key x: {:?}, y: {:?}",
            -alice_public_key.get_x(),
            -alice_public_key.get_y()
        );

        // 4. verify proof
        let verify_proof = start_timer!(|| "verify proof");
        verifier
            .verify(&proof, &public_inputs)
            .expect("failed to verify proof");
        end_timer!(verify_proof);
    }
}
