use crate::mock::new_test_ext;
use crate::pallet::Config;
use crate::{self as zkrollup_pallet};

use bls_12_381::Fr;
use ec_pairing::TatePairing;
use frame_support::{construct_runtime, parameter_types};
use jub_jub::JubjubAffine;
use red_jubjub::{PublicKey, RedJubjub};
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};
use zkrollup::{Batch, BatchCircuit, Poseidon, Transaction};
use zkstd::common::Pairing;

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
    type Pairing = TatePairing;
    type Affine = JubjubAffine;
    type CustomCircuit = BatchCircuit<RedJubjub, Poseidon<Fr, 2>, TREE_HEIGH, BATCH_SIZE>;
    type Event = Event;
}

impl Config for TestRuntime {
    type Transaction = Transaction<Self::RedDsa>;
    type PublicKey = PublicKey<Self::RedDsa>;
    type Event = Event;
    type Plonk = Plonk;
    type RedDsa = RedJubjub;
    type Batch = Batch<
        Self::RedDsa,
        Poseidon<<<Self as pallet_plonk::Config>::Pairing as Pairing>::ScalarField, 2>,
        TREE_HEIGH,
        BATCH_SIZE,
    >;
}

#[cfg(test)]
mod zkrollup_tests {
    use super::*;
    use crate::traits::Rollup;
    use frame_support::assert_ok;
    use jub_jub::Fp;
    use pallet_plonk::FullcodecRng;
    use rand::SeedableRng;
    use red_jubjub::SecretKey;
    use zkrollup::{BatchGetter, Poseidon, RollupOperator, TransactionData};
    use zkstd::common::Group;

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

        new_test_ext().execute_with(|| {
            // 2. Generate trusted setup
            assert_ok!(ZkRollup::trusted_setup(
                operator_origin.clone(),
                15,
                rng.clone()
            ));
            let pp = Plonk::public_params().unwrap();

            // 3. Create an operator
            let mut operator = RollupOperator::<
                RedJubjub,
                TatePairing,
                Poseidon<Fr, 2>,
                TREE_HEIGH,
                BATCH_SIZE,
            >::new(Poseidon::<Fr, 2>::new(), pp);

            // Assures that null elements' hashes are correct
            assert_eq!(
                operator.state_root(),
                Fr::from_hex("0x67289036dacea08e3f51549e382ee81b28b3206bf5053d8f2b723d73c73e06ab")
                    .unwrap()
            );

            // 4. Set initial root on L1
            assert_ok!(ZkRollup::set_initial_root(
                operator_origin.clone(),
                operator.state_root()
            ));

            assert_eq!(<ZkRollup as Rollup>::state_root(), operator.state_root());

            // 5. Create and sign deposit transactions
            let deposit1 = (10, alice_address);
            let deposit2 = (0, bob_address);

            // 6. Add them to the deposit pool on the L1
            assert_ok!(ZkRollup::deposit(alice_origin, deposit1.0, deposit1.1));

            // 7. Explicitly process data on L2. Will be changed, when communication between layers will be decided.
            operator.process_deposit(deposit1.0, deposit1.1);
            // }

            assert_eq!(
                operator.state_root(),
                Fr::from_hex("0x35e253ed42df14f4ec76ab96402adc4971e51d00403373354ba36414f26d4c08")
                    .unwrap()
            );

            // Same for the second deposit
            assert_ok!(ZkRollup::deposit(bob_origin, deposit2.0, deposit2.1));
            operator.process_deposit(deposit2.0, deposit2.1);

            assert_eq!(
                operator.state_root(),
                Fr::from_hex("0x4faf040d99b142bfd80ac803b5c2e6ad4eff898327f6f075b8efacbaa3283691")
                    .unwrap()
            );

            // 8. Prepared and sign transfer transactions
            let t1 =
                TransactionData::new(alice_address, bob_address, 10).signed(alice_secret, &mut rng);
            let t2 =
                TransactionData::new(bob_address, alice_address, 5).signed(bob_secret, &mut rng);

            // 9. Execute transactions on L2
            assert!(operator.execute_transaction(t1).is_none());

            assert_eq!(
                operator.state_root(),
                Fr::from_hex("0x178608320947801f4aa5159136664790fd7b68bdd0d2646794990fc075688f73")
                    .unwrap()
            );

            // With BATCH_SIZE == 2 second transaction should create a proof and batch
            let ((proof, public_inputs), batch) = operator.execute_transaction(t2).unwrap();
            assert_eq!(
                operator.state_root(),
                Fr::from_hex("0x454e8c44dcbc3b955ac320345cc23645bd0ba8638dfb1b02b00651896f218ed3")
                    .unwrap()
            );

            // 10. Explicitly add_batch on L1. Will be changed, when communication between layers will be decided.

            assert_ok!(ZkRollup::update_state(
                operator_origin,
                proof,
                public_inputs,
                batch
            ));

            // 11. Check that state root on L1 changed.
            assert_eq!(<ZkRollup as Rollup>::state_root(), operator.state_root());

            // Withdraw

            // 1. Burn funds on L2 by sending to a special address
            let alice_withdraw: Transaction<RedJubjub> =
                TransactionData::new(alice_address, PublicKey::zero(), 5)
                    .signed(alice_secret, &mut rng);
            let bob_withdraw: Transaction<RedJubjub> =
                TransactionData::new(bob_address, PublicKey::zero(), 5)
                    .signed(bob_secret, &mut rng);

            operator.execute_transaction(alice_withdraw);
            let (proof, batch) = operator.execute_transaction(bob_withdraw).unwrap();

            assert_eq!(
                batch.withdraw_info(),
                [(5, alice_address), (5, bob_address)]
            );
        });
    }
}
