use jub_jub::{Fp as JubJubScalar, JubjubAffine};
use she_elgamal::{ConfidentialTransferPublicInputs, EncryptedNumber};
use zkplonk::prelude::*;
use zkstd::common::{Decode, Encode, Group, TwistedEdwardsAffine, TwistedEdwardsCurve};

pub const BALANCE_BITS: usize = 16;
pub const CONFIDENTIAL_TRANSFER_PUBLIC_INPUT_LENGTH: usize = 8;

/// Confidential transfer circuit
#[derive(Debug, PartialEq)]
pub struct ConfidentialTransferCircuit {
    sender_public_key: JubjubAffine,
    recipient_public_key: JubjubAffine,
    sender_encrypted_balance: EncryptedNumber,
    sender_encrypted_transfer_amount: EncryptedNumber,
    recipient_encrypted_transfer_amount: JubjubAffine,
    sender_private_key: JubJubScalar,
    transfer_amount: JubJubScalar,
    sender_after_balance: JubJubScalar,
    randomness: JubJubScalar,
    bits: usize,
}

impl ConfidentialTransferCircuit {
    /// Init confidential tranfer circuit
    #[allow(dead_code)]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        sender_public_key: JubjubAffine,
        recipient_public_key: JubjubAffine,
        sender_encrypted_balance: EncryptedNumber,
        sender_encrypted_transfer_amount: EncryptedNumber,
        recipient_encrypted_transfer_amount: JubjubAffine,
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
            sender_public_key: JubjubAffine::ADDITIVE_IDENTITY,
            recipient_public_key: JubjubAffine::ADDITIVE_IDENTITY,
            sender_encrypted_balance: EncryptedNumber::default(),
            sender_encrypted_transfer_amount: EncryptedNumber::default(),
            recipient_encrypted_transfer_amount: JubjubAffine::ADDITIVE_IDENTITY,
            sender_private_key: JubJubScalar::ADDITIVE_IDENTITY,
            transfer_amount: JubJubScalar::ADDITIVE_IDENTITY,
            sender_after_balance: JubJubScalar::ADDITIVE_IDENTITY,
            randomness: JubJubScalar::ADDITIVE_IDENTITY,
            bits: BALANCE_BITS,
        }
    }
}

impl Circuit<JubjubAffine> for ConfidentialTransferCircuit {
    type ConstraintSystem = Plonk<JubjubAffine>;
    fn synthesize(&self, composer: &mut Plonk<JubjubAffine>) -> Result<(), Error> {
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
        let g_pow_balance = composer
            .component_mul_generator(transfer_amount, JubjubExtended::ADDITIVE_GENERATOR)?;
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
        let g_pow_randomness =
            composer.component_mul_generator(randomness, JubjubExtended::ADDITIVE_GENERATOR)?;
        composer.assert_equal_public_point(g_pow_randomness, alice_s_transfer_amount);

        // Alice after balance check
        let g_pow_after_balance = composer
            .component_mul_generator(sender_after_balance, JubjubExtended::ADDITIVE_GENERATOR)?;
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
        let calculated_pk = composer
            .component_mul_generator(sender_private_key, JubjubExtended::ADDITIVE_GENERATOR)?;
        composer.assert_equal_public_point(calculated_pk, self.sender_public_key);

        // Transfer amount and ramaining balance range check
        composer.component_range(transfer_amount, self.bits);
        composer.component_range(sender_after_balance, self.bits);

        Ok(())
    }
}

/// confidential transfer transaction input
#[derive(Clone, Debug, Decode, Encode, Eq, PartialEq)]
pub struct ConfidentialTransferTransaction<
    E: ConfidentialTransferPublicInputs<A>,
    A: TwistedEdwardsAffine,
> {
    /// sender public key
    pub sender_public_key: A,
    /// recipient public key
    pub recipient_public_key: A,
    /// encrypted transfer amount by sender
    pub sender_encrypted_transfer_amount: E,
    /// encrypted transfer amount by recipient
    pub recipient_encrypted_transfer_amount: A,
    /// the other encrypted transfer amount by recipient
    pub recipient_encrypted_transfer_amount_other: A,
}

impl<E: ConfidentialTransferPublicInputs<A>, A: TwistedEdwardsAffine>
    ConfidentialTransferTransaction<E, A>
{
    /// init confidential transfer transaction
    pub fn new(
        sender_public_key: A,
        recipient_public_key: A,
        sender_encrypted_transfer_amount: E,
        recipient_encrypted_transfer_amount: A,
        recipient_encrypted_transfer_amount_other: A,
    ) -> Self {
        Self {
            sender_public_key,
            recipient_public_key,
            sender_encrypted_transfer_amount,
            recipient_encrypted_transfer_amount,
            recipient_encrypted_transfer_amount_other,
        }
    }

    /// output public inputs for confidential transfer transaction
    pub fn public_inputs(self) -> [A::Range; CONFIDENTIAL_TRANSFER_PUBLIC_INPUT_LENGTH] {
        let mut public_inputs = [A::Range::zero(); CONFIDENTIAL_TRANSFER_PUBLIC_INPUT_LENGTH];
        let (sender_t, sender_s) = self.sender_encrypted_transfer_amount.get();
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
            public_inputs[i * 2 + 1] = y;
        }
        public_inputs
    }

    /// output transfer amount encrypted for each
    pub fn transaction_amount(self) -> (E, E) {
        (
            self.sender_encrypted_transfer_amount,
            E::init(
                self.recipient_encrypted_transfer_amount,
                self.recipient_encrypted_transfer_amount_other,
            ),
        )
    }
}
