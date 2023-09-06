use crate::mock::new_test_ext;
use crate::{self as main_contract};
use crate::{pallet::Config, types::*};

use frame_support::{construct_runtime, parameter_types};
use jub_jub::Fp;
use pallet_zk_rollup::{Batch, Poseidon, Proof, Transaction};
use red_jubjub::PublicKey;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;
type Block = frame_system::mocking::MockBlock<TestRuntime>;

construct_runtime!(
    pub enum TestRuntime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Module, Call, Config, Storage, Event<T>},
        MainContract: main_contract::{Module, Call, Storage, Event<T>},
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

impl Config for TestRuntime {
    type F = Fp;
    type Transaction = Transaction;

    type Batch = Batch<Self::F, Poseidon<Self::F, 2>, 2, 2>;

    type Proof = Proof<Self::F, Poseidon<Self::F, 2>, 2, 2>;

    type PublicKey = PublicKey;
    type Event = Event;
}

// SBP-M1 review: poor testing, missing coverage for edge cases, errors, etc.
#[cfg(test)]
mod main_contract_test {
    use crate::traits::MainContract;

    use super::*;
    use jub_jub::{Fp, JubjubExtended};
    use pallet_zk_rollup::{Poseidon, RollupOperator, TransactionData};
    use rand::{rngs::StdRng, SeedableRng};
    use red_jubjub::SecretKey;
    use zkstd::{behave::Group, common::CurveGroup};

    // fn get_rng() -> FullcodecRng {
    //     FullcodecRng::from_seed([
    //         0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06,
    //         0xbc, 0xe5,
    //     ])
    // }

    #[test]
    fn default_test() {
        // let rng = get_rng();
        let mut rng = StdRng::seed_from_u64(8349u64);
        const ACCOUNT_LIMIT: usize = 2;
        const BATCH_SIZE: usize = 2;

        // 1. Create an operator and contract

        let mut operator = RollupOperator::<Fp, Poseidon<Fp, 2>, ACCOUNT_LIMIT, BATCH_SIZE>::new(
            Poseidon::<Fp, 2>::new(),
        );

        // 2. Generate user data
        let alice_secret = SecretKey::new(Fp::random(&mut rng));
        let bob_secret = SecretKey::new(Fp::random(&mut rng));
        let alice_address = alice_secret.to_public_key();
        let bob_address = bob_secret.to_public_key();
        // Decided by the operator
        let withdraw_address = PublicKey::new(JubjubExtended::random(&mut rng));
        // Will be changed. Implementation for test
        operator.add_withdrawal_address(withdraw_address);
        // State root will be changed here, but we can ignore it.

        new_test_ext().execute_with(|| {
            let mut rng = StdRng::seed_from_u64(8349u64);

            // let mut contract = MainContract::<Fp, Poseidon<Fp, 2>, ACCOUNT_LIMIT, BATCH_SIZE>::new(
            //     operator.state_root(),
            //     PublicKey::new(JubjubExtended::random(&mut rng)),
            // );

            // Assures that null elements' hashes are correct
            let root_before_dep = operator.state_root();
            assert_eq!(
                root_before_dep,
                Fp::from_hex("0x082e6d1a102e14de34bf3471c6a79c4ae3069fbaad7346032d40626576cf4039")
                    .unwrap()
            );

            // 3. Create and sign deposit transactions
            let deposit1 = TransactionData::new(alice_address, MainContract::address(), 10)
                .signed(alice_secret, &mut rng);
            let deposit2 = TransactionData::new(bob_address, MainContract::address(), 0)
                .signed(bob_secret, &mut rng);

            // 4. Add them to the deposit pool on the L1
            MainContract::deposit(deposit1);
            MainContract::deposit(deposit2);

            let pending_deposits = vec![];
            assert_eq!(pending_deposits.len(), 2);
            // 5. Explicitly process data on L2. Will be changed, when communication between layers will be decided.
            operator.process_deposits(pending_deposits.clone());

            // Assures that deposits were processed on L2 and state tree was changed.
            let root_after_dep = operator.state_root();
            assert_eq!(
                root_after_dep,
                Fp::from_hex("0x0e19d7c5c79887947f8f9e73f07570eaabc7a4d2f5efb1c34b0b5d40e63ec4d1")
                    .unwrap()
            );

            // Need to implement balance verification for users through the contract
            // assert!(contract.check_balance(MerkleProof::default()) == alice.balance());
            // assert!(contract.check_balance(MerkleProof::default()) == bob.balance()));

            // 6. Prepared and sign transfer transactions
            let t1 =
                TransactionData::new(alice_address, bob_address, 10).signed(alice_secret, &mut rng);
            let t2 =
                TransactionData::new(bob_address, alice_address, 5).signed(bob_secret, &mut rng);

            // 7. Execute transactions on L2
            assert!(operator.execute_transaction(t1).is_none());
            // With BATCH_SIZE == 2 second transaction should create a proof and batch
            let (proof, batch) = operator.execute_transaction(t2).unwrap();
            let root_after_tx = operator.state_root();
            // State root should change as wells
            assert_eq!(
                root_after_tx,
                Fp::from_hex("0x0e19d7c5c79887947f8f9e73f07570eaabc7a4d2f5efb1c34b0b5d40e63ec4d1")
                    .unwrap()
            );

            // 8. Explicitly add_batch on L1. Will be changed, when communication between layers will be decided.
            MainContract::add_batch(proof, batch);

            // 9. Check that batch info is on L1.
            let txs: Vec<Transaction> = batch.raw_transactions().cloned().collect();
            let expected_txs = vec![t1, t2];
            assert_eq!(&txs, &expected_txs);
            assert_eq!(batch.border_roots(), (root_after_dep, root_after_tx));
            // 10. Check that state root on L1 changed.
            assert_eq!(MainContract::state_root(), root_after_tx);
        });
    }
}
