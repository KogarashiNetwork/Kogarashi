#[cfg(test)]
mod plonk_test {
    use crate::circuit::ConfidentialTransferCircuit;
    use crate::mock::{
        generate_confidential_transfer_params, new_test_ext, ConfidentialTransfer, Origin, Plonk,
        ALICE_ADDRESS, ALICE_AFTER_BALANCE, ALICE_BALANCE, ALICE_PRIVATE_KEY, BOB_ADDRESS,
        BOB_AFTER_BALANCE, BOB_BALANCE, BOB_PRIVATE_KEY,
    };
    use crate::traits::ConfidentialTransfer as TraitConfidentialTransfer;

    use ark_std::{end_timer, start_timer};
    use ec_pairing::TatePairing;
    use frame_support::assert_ok;
    use jub_jub::JubjubAffine;
    use pallet_plonk::FullcodecRng;
    use rand::SeedableRng;
    use zkplonk::prelude::PlonkKey;
    use zksnarks::keypair::Keypair;

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

        let (confidential_transfer_circuit, confidential_transfer_transaction) =
            generate_confidential_transfer_params();

        new_test_ext().execute_with(|| {
            // default balance decryption check
            let alice_balance = ConfidentialTransfer::total_balance(&ALICE_ADDRESS);
            let alice_raw_balance = alice_balance.decrypt(ALICE_PRIVATE_KEY);
            let bob_balance = ConfidentialTransfer::total_balance(&BOB_ADDRESS);
            let bob_raw_balance = bob_balance.decrypt(BOB_PRIVATE_KEY);

            assert_eq!(alice_raw_balance.unwrap(), ALICE_BALANCE);
            assert_eq!(bob_raw_balance.unwrap(), BOB_BALANCE);

            // trusted setup check
            let trusted_setup = start_timer!(|| "trusted setup");
            let result =
                ConfidentialTransfer::trusted_setup(Origin::signed(ALICE_ADDRESS), k, rng.clone());
            end_timer!(trusted_setup);
            assert_ok!(result);

            // proof generation
            let mut pp = Plonk::public_params().unwrap();
            let prover =
                PlonkKey::<TatePairing, JubjubAffine, ConfidentialTransferCircuit>::compile(
                    &mut pp,
                )
                .expect("failed to compile circuit");

            let proof_generation = start_timer!(|| "proof generation");
            let proof = prover
                .0
                .create_proof(&mut rng, &confidential_transfer_circuit)
                .expect("failed to prove");
            end_timer!(proof_generation);

            // confidential transfer check
            let confidential_transfer = start_timer!(|| "confidential transfer");
            let result = ConfidentialTransfer::confidential_transfer(
                Origin::signed(ALICE_ADDRESS),
                BOB_ADDRESS,
                proof.0,
                confidential_transfer_transaction,
            );
            end_timer!(confidential_transfer);
            assert_ok!(result);

            // balance transition check
            let alice_balance = ConfidentialTransfer::total_balance(&ALICE_ADDRESS);
            let alice_raw_balance = alice_balance.decrypt(ALICE_PRIVATE_KEY);
            let bob_balance = ConfidentialTransfer::total_balance(&BOB_ADDRESS);
            let bob_raw_balance = bob_balance.decrypt(BOB_PRIVATE_KEY);

            assert_eq!(alice_raw_balance.unwrap(), ALICE_AFTER_BALANCE);
            assert_eq!(bob_raw_balance.unwrap(), BOB_AFTER_BALANCE);
        });
    }
}
