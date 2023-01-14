#[cfg(test)]
mod plonk_test {
    use crate::mock::{
        generate_confidential_transfer_params, new_test_ext, ConfidentialTransfer, Origin, Plonk,
    };

    use frame_support::assert_ok;
    use pallet_plonk::FullcodecRng;
    use rand::SeedableRng;
    use zero_circuits::ConfidentialTransferCircuit;
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

        let (confidential_transfer_circuit, confidential_transfer_transaction) =
            generate_confidential_transfer_params(&mut rng);

        new_test_ext().execute_with(|| {
            assert_ok!(ConfidentialTransfer::trusted_setup(
                Origin::signed(1),
                k,
                get_rng()
            ));
            let pp = Plonk::public_parameter().unwrap();

            let (prover, _) = Compiler::compile::<ConfidentialTransferCircuit>(&pp, label)
                .expect("failed to compile circuit");

            let (proof, _) = prover
                .prove(&mut rng, &confidential_transfer_circuit)
                .expect("failed to prove");

            assert_ok!(ConfidentialTransfer::confidential_transfer(
                Origin::signed(1),
                2,
                proof,
                confidential_transfer_transaction
            ));
        });
    }
}
