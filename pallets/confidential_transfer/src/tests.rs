#[cfg(test)]
mod plonk_test {
    use crate::mock::{new_test_ext, ConfidentialTransfer, Origin, Plonk};

    use frame_support::assert_ok;
    use pallet_plonk::{FullcodecRng, JubJubAffine, JubJubScalar, GENERATOR_EXTENDED};
    use rand::SeedableRng;
    use zero_circuits::{ConfidentialTransferCircuit, ConfidentialTransferTransaction};
    use zero_crypto::behave::Group;
    use zero_elgamal::EncryptedNumber;
    use zero_plonk::prelude::Compiler;

    fn get_rng() -> FullcodecRng {
        FullcodecRng::from_seed([
            0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06,
            0xbc, 0xe5,
        ])
    }

    #[test]
    fn confidential_transfer_test() {
        let k = 14;
        let mut rng = get_rng();
        let label = b"verify";

        let generator = GENERATOR_EXTENDED;
        let alice_private_key = JubJubScalar::random(&mut rng);
        let bob_private_key = JubJubScalar::random(&mut rng);
        let alice_public_key = generator * alice_private_key;
        let bob_public_key = generator * bob_private_key;

        let alice_balance = JubJubScalar::from(1500 as u64);
        let transfer_amount = JubJubScalar::from(800 as u64);
        let alice_after_balance = JubJubScalar::from(700 as u64);
        let alice_original_randomness = JubJubScalar::from(789 as u64);
        let randomness = JubJubScalar::from(123 as u64);

        let alice_left_encrypted_balance =
            (generator * alice_balance) + (alice_public_key * alice_original_randomness);
        let alice_right_encrypted_balance = generator * alice_original_randomness;
        let alice_left_encrypted_transfer_amount =
            (generator * transfer_amount) + (alice_public_key * randomness);
        let alice_right_encrypted_transfer_amount = generator * randomness;
        let bob_encrypted_transfer_amount =
            (generator * transfer_amount) + (bob_public_key * randomness);
        let alice_encrypted_transfer_amount = EncryptedNumber::new(
            JubJubAffine::from(alice_left_encrypted_transfer_amount),
            JubJubAffine::from(alice_right_encrypted_transfer_amount),
        );
        let alice_public_key = alice_public_key.into();
        let bob_public_key = bob_public_key.into();
        let bob_encrypted_transfer_amount = bob_encrypted_transfer_amount.into();

        let transaction_params = ConfidentialTransferTransaction {
            sender_public_key: alice_public_key,
            recipient_public_key: bob_public_key,
            sender_encrypted_transfer_amount: alice_encrypted_transfer_amount,
            recipient_encrypted_transfer_amount: bob_encrypted_transfer_amount,
        };

        new_test_ext().execute_with(|| {
            assert_ok!(ConfidentialTransfer::trusted_setup(
                Origin::signed(1),
                k,
                get_rng()
            ));
            let pp = Plonk::public_parameter().unwrap();

            let (prover, _) = Compiler::compile::<ConfidentialTransferCircuit>(&pp, label)
                .expect("failed to compile circuit");

            let (proof, public_inputs) = prover
                .prove(
                    &mut rng,
                    &ConfidentialTransferCircuit::new(
                        alice_public_key,
                        bob_public_key,
                        EncryptedNumber::new(
                            JubJubAffine::from(alice_left_encrypted_balance),
                            JubJubAffine::from(alice_right_encrypted_balance),
                        ),
                        EncryptedNumber::new(
                            JubJubAffine::from(alice_left_encrypted_transfer_amount),
                            JubJubAffine::from(alice_right_encrypted_transfer_amount),
                        ),
                        bob_encrypted_transfer_amount,
                        alice_private_key,
                        transfer_amount,
                        alice_after_balance,
                        randomness,
                    ),
                )
                .expect("failed to prove");

            assert_ok!(ConfidentialTransfer::confidential_transfer(
                Origin::signed(1),
                2,
                proof,
                public_inputs,
                transaction_params
            ));
        });
    }
}
