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
use zkrollup::{Batch, BatchCircuit, Poseidon, Proof, Transaction};

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
    type CustomCircuit = BatchCircuit<TatePairing, Poseidon<Fr, 2>, 2, 2>;
    type Event = Event;
}

impl Config for TestRuntime {
    type F = Fr;
    type Transaction = Transaction<TatePairing>;

    type Batch = Batch<TatePairing, Poseidon<Self::F, 2>, 2, 2>;

    type Proof = Proof<Self::F, Poseidon<Self::F, 2>, 2, 2>;

    type PublicKey = PublicKey<TatePairing>;
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
    use rand::{rngs::StdRng, SeedableRng};
    use red_jubjub::SecretKey;
    use zkrollup::{Poseidon, RollupOperator, TransactionData};
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
        const ACCOUNT_LIMIT: usize = 2;
        const BATCH_SIZE: usize = 2;

        let main_contract_address = PublicKey::new(JubjubExtended::random(&mut rng));
        let operator_origin = Origin::signed(3);
        assert_ok!(ZkRollup::trusted_setup(
            operator_origin.clone(),
            15,
            rng.clone()
        ));
        let pp = Plonk::keypair().unwrap();

        let mut operator =
            RollupOperator::<TatePairing, Poseidon<Fr, 2>, ACCOUNT_LIMIT, BATCH_SIZE>::new(
                Poseidon::<Fr, 2>::new(),
                pp,
            );

        let alice_secret = SecretKey::new(Fp::random(&mut rng));
        let alice_origin = Origin::signed(1);
        let bob_secret = SecretKey::new(Fp::random(&mut rng));
        let bob_origin = Origin::signed(2);
        let alice_address = alice_secret.to_public_key();
        let bob_address = bob_secret.to_public_key();
        let withdraw_address = PublicKey::new(JubjubExtended::random(&mut rng));

        operator.add_withdrawal_address(withdraw_address);

        new_test_ext().execute_with(|| {
            let mut rng = StdRng::seed_from_u64(8349u64);

            let deposit1 = TransactionData::new(alice_address, main_contract_address, 10)
                .signed(alice_secret, &mut rng);
            let deposit2 = TransactionData::new(bob_address, main_contract_address, 0)
                .signed(bob_secret, &mut rng);

            assert_ok!(ZkRollup::deposit(alice_origin, deposit1));

            // let deposit = events();
            // assert_eq!(
            //     deposit,
            //     [Event::main_contract(crate::Event::Deposit(deposit1)),]
            // );
            // if let Event::main_contract(crate::Event::Deposit(t)) = deposit.first().unwrap() {
            operator.process_deposit(deposit1);
            // }

            assert_ok!(ZkRollup::deposit(bob_origin, deposit2));
            // let deposit = events();
            // assert_eq!(
            //     deposit,
            //     [Event::main_contract(crate::Event::Deposit(deposit2)),]
            // );
            // if let Event::main_contract(crate::Event::Deposit(t)) = deposit.first().unwrap() {
            operator.process_deposit(deposit2);
            // }

            let t1 =
                TransactionData::new(alice_address, bob_address, 10).signed(alice_secret, &mut rng);
            let t2 =
                TransactionData::new(bob_address, alice_address, 5).signed(bob_secret, &mut rng);

            assert!(operator.execute_transaction(t1).is_none());
            let (proof, batch) = operator.execute_transaction(t2).unwrap();
            let root_after_tx = operator.state_root();

            assert_ok!(ZkRollup::update_state(operator_origin, proof, batch));
            // assert_eq!(
            //     events(),
            //     [Event::main_contract(crate::Event::StateUpdated(
            //         root_after_tx
            //     )),]
            // );

            // 10. Check that state root on L1 changed.
            assert_eq!(<ZkRollup as Rollup>::state_root(), root_after_tx);
        });
    }
}
