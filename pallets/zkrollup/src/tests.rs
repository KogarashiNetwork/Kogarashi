use crate::mock::new_test_ext;
use crate::pallet::Config;
use crate::{self as zkrollup_pallet};

use bls_12_381::Fr;
use ec_pairing::TatePairing;
use frame_support::{construct_runtime, parameter_types};
use red_jubjub::PublicKey;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};
use zkrollup::{Batch, BatchCircuit, Poseidon, Transaction};
use zkstd::common::Pairing;

// let last_level_size = leaves.len().next_power_of_two();
// let tree_size = 2 * last_level_size - 1;
// let tree_height = tree_height(tree_size as u64);
const TREE_HEIGH: usize = 3;
// Need to specify the size of tree as well
const BATCH_SIZE: usize = 2;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;
type Block = frame_system::mocking::MockBlock<TestRuntime>;

construct_runtime!(
    pub enum TestRuntime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Module, Call, Config, Storage, Event<T>},
        Plonk: pallet_plonk::{Module, Call, Storage, Event<T>},
        ZkRollup: zkrollup_pallet::{Module, Call, Storage, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::simple_max(1024);
}

impl frame_system::Config for TestRuntime {
    type BaseCallFilter = ();
    type BlockWeights = ();
    type BlockLength = ();
    type Origin = Origin;
    type Index = u64;
    type Call = Call;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type DbWeight = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
}

impl pallet_plonk::Config for TestRuntime {
    type P = TatePairing;
    type CustomCircuit = BatchCircuit<TatePairing, Poseidon<Fr, 2>, TREE_HEIGH, BATCH_SIZE>;
    type Event = Event;
}

impl Config for TestRuntime {
    type Transaction = Transaction<<Self as pallet_plonk::Config>::P>;

    type Batch = Batch<
        <Self as pallet_plonk::Config>::P,
        Poseidon<<<Self as pallet_plonk::Config>::P as Pairing>::ScalarField, 2>,
        TREE_HEIGH,
        BATCH_SIZE,
    >;

    type PublicKey = PublicKey<<Self as pallet_plonk::Config>::P>;
    type Event = Event;
    type Plonk = Plonk;
}

#[cfg(test)]
mod zkrollup_tests {
    use super::*;
    use crate::traits::Rollup;
    use frame_support::assert_ok;
    use jub_jub::{Fp, JubjubExtended};
    use pallet_plonk::FullcodecRng;
    use rand::SeedableRng;
    use red_jubjub::SecretKey;
    use zkrollup::{BatchGetter, Poseidon, RollupOperator, TransactionData};
    use zkstd::{behave::Group, common::CurveGroup};

    // fn events() -> Vec<Event> {
    //     let evt = System::events()
    //         .into_iter()
    //         .map(|evt| evt.event)
    //         .collect::<Vec<_>>();

    //     System::reset_events();

    //     evt
    // }

    fn get_rng() -> FullcodecRng {
        FullcodecRng::from_seed([
            0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06,
            0xbc, 0xe5,
        ])
    }

    #[test]
    fn zkrollup_test() {
        let mut rng = get_rng();

        // 1. Generate user data
        let operator_origin = Origin::signed(3);
        let alice_secret = SecretKey::new(Fp::random(&mut rng));
        let alice_origin = Origin::signed(1);
        let bob_secret = SecretKey::new(Fp::random(&mut rng));
        let bob_origin = Origin::signed(2);
        let alice_address = alice_secret.to_public_key();
        let bob_address = bob_secret.to_public_key();
        let withdraw_address = PublicKey::new(JubjubExtended::random(&mut rng));
        // Address to which we deposit funds as user
        let main_contract_address = PublicKey::new(JubjubExtended::random(&mut rng));

        new_test_ext().execute_with(|| {
            // 2. Generate trusted setup
            assert_ok!(ZkRollup::trusted_setup(
                operator_origin.clone(),
                15,
                rng.clone()
            ));
            let pp = Plonk::keypair().unwrap();

            // 3. Create an operator
            let mut operator =
                RollupOperator::<TatePairing, Poseidon<Fr, 2>, TREE_HEIGH, BATCH_SIZE>::new(
                    Poseidon::<Fr, 2>::new(),
                    pp,
                );

            // Assures that null elements' hashes are correct
            assert_eq!(
                operator.state_root(),
                Fr::from_hex("0x0000000000000000000000000000000000000000000000000000000011a2197e")
                    .unwrap()
            );

            // Decided by the operator
            operator.add_withdraw_address(withdraw_address);

            // State root will be changed here
            assert_eq!(
                operator.state_root(),
                Fr::from_hex("0x4ac4ba7fd748d1dac2984fed7389b62860ab9f2b9feef22cbbb62ad02c4448fd")
                    .unwrap()
            );

            // 4. Set initial root on L1
            assert_ok!(ZkRollup::set_initial_root(
                operator_origin.clone(),
                operator.state_root()
            ));

            assert_eq!(<ZkRollup as Rollup>::state_root(), operator.state_root());

            // 5. Create and sign deposit transactions
            let deposit1 = TransactionData::new(alice_address, main_contract_address, 10)
                .signed(alice_secret, &mut rng);
            let deposit2 = TransactionData::new(bob_address, main_contract_address, 0)
                .signed(bob_secret, &mut rng);

            // 6. Add them to the deposit pool on the L1
            assert_ok!(ZkRollup::deposit(alice_origin, deposit1));

            // let deposit = events();
            // assert_eq!(
            //     deposit,
            //     [Event::main_contract(crate::Event::Deposit(deposit1)),]
            // );
            // if let Event::main_contract(crate::Event::Deposit(t)) = deposit.first().unwrap() {

            // 7. Explicitly process data on L2. Will be changed, when communication between layers will be decided.
            operator.process_deposit(deposit1);
            // }

            assert_eq!(
                operator.state_root(),
                Fr::from_hex("0x6de0dc9479e61cefda6be0f704f077060d574e5322ef5546a5cb4f9802ca1c23")
                    .unwrap()
            );

            // Same for the second deposit
            assert_ok!(ZkRollup::deposit(bob_origin, deposit2));
            // let deposit = events();
            // assert_eq!(
            //     deposit,
            //     [Event::main_contract(crate::Event::Deposit(deposit2)),]
            // );
            // if let Event::main_contract(crate::Event::Deposit(t)) = deposit.first().unwrap() {
            operator.process_deposit(deposit2);
            // }

            assert_eq!(
                operator.state_root(),
                Fr::from_hex("0x2613603990a71c155983132f0a2df05694f6dcedca646e2da307d451e6610a2f")
                    .unwrap()
            );

            // Need to implement balance verification for users through the contract
            // assert!(contract.check_balance(MerkleProof::default()) == alice.balance());
            // assert!(contract.check_balance(MerkleProof::default()) == bob.balance()));

            // 8. Prepared and sign transfer transactions
            let t1 =
                TransactionData::new(alice_address, bob_address, 10).signed(alice_secret, &mut rng);
            let t2 =
                TransactionData::new(bob_address, alice_address, 5).signed(bob_secret, &mut rng);

            // 9. Execute transactions on L2
            assert!(operator.execute_transaction(t1).is_none());

            assert_eq!(
                operator.state_root(),
                Fr::from_hex("0x62b2341b616b111be03b16ffcb13724c849c5764538dd1074e742d6ace20a88e")
                    .unwrap()
            );

            // With BATCH_SIZE == 2 second transaction should create a proof and batch
            let ((proof, public_inputs), batch) = operator.execute_transaction(t2).unwrap();
            assert_eq!(
                operator.state_root(),
                Fr::from_hex("0x0d27ef4dd857ef830c23668ad14a2dff40038972e1dd26660c3240e6f0b2fd4e")
                    .unwrap()
            );

            // 10. Explicitly add_batch on L1. Will be changed, when communication between layers will be decided.

            assert_ok!(ZkRollup::update_state(
                operator_origin,
                proof,
                public_inputs,
                batch
            ));
            // assert_eq!(
            //     events(),
            //     [Event::main_contract(crate::Event::StateUpdated(
            //         root_after_tx
            //     )),]
            // );

            // 11. Check that state root on L1 changed.
            assert_eq!(<ZkRollup as Rollup>::state_root(), operator.state_root());

            // Withdraw

            // 1. Burn funds on L2 by sending to a special address
            let alice_withdraw: Transaction<TatePairing> =
                TransactionData::new(alice_address, withdraw_address, 5)
                    .signed(alice_secret, &mut rng);
            let bob_withdraw: Transaction<TatePairing> =
                TransactionData::new(bob_address, withdraw_address, 5).signed(bob_secret, &mut rng);

            operator.execute_transaction(alice_withdraw);
            let (proof, batch) = operator.execute_transaction(bob_withdraw).unwrap();

            assert_eq!(
                batch.withdraw_info(),
                [(5, alice_address), (5, bob_address)]
            );

            // // 2. l2_burn_merkle_proof_alice and l2_burn_merkle_proof_bob should be generated with batch_tree
            // // Will decide the process, while implementing the gadget

            // // 3. Withdraw on L1
            // contract.withdraw(
            //     // l2_burn_merkle_proof_alice,
            //     batch.final_root(),
            //     alice_withdraw,
            //     alice_address,
            // );

            // contract.withdraw(
            //     // l2_burn_merkle_proof_bob,
            //     batch.final_root(),
            //     bob_withdraw,
            //     bob_address,
            // );
        });
    }
}
