use crate::{self as sum_storage, client::ExampleFunction, Config};

use frame_support::assert_ok;
use frame_support::parameter_types;
use frame_system as system;
use pallet_nova::*;
use rand_core::SeedableRng;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;
type Block = frame_system::mocking::MockBlock<TestRuntime>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum TestRuntime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Module, Call, Config, Storage, Event<T>},
        Nova: pallet_nova::{Module, Call, Storage},
        SumStorage: sum_storage::{Module, Call, Storage, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::simple_max(1024);
}

impl system::Config for TestRuntime {
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

impl pallet_nova::Config for TestRuntime {
    type E1 = Bn254Driver;
    type E2 = GrumpkinDriver;
    type FC1 = ExampleFunction<Fr>;
    type FC2 = ExampleFunction<Fq>;
}

impl Config for TestRuntime {
    type Event = Event;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default()
        .build_storage::<TestRuntime>()
        .unwrap()
        .into()
}

fn get_rng() -> FullcodecRng {
    FullcodecRng::from_seed([
        0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06, 0xbc,
        0xe5,
    ])
}

#[test]
fn default_sum_zero() {
    new_test_ext().execute_with(|| {
        assert_eq!(SumStorage::get_sum(), 0);
    });
}

/// The set `Thing1` storage with valid proof
#[test]
fn sums_thing_one() {
    let mut rng = get_rng();

    let pp = PublicParams::<
            Bn254Driver,
            GrumpkinDriver,
            ExampleFunction<Fr>,
            ExampleFunction<Fq>,
        >::setup(&mut rng);

    let z0_primary = DenseVectors::new(vec![Fr::from(0)]);
    let z0_secondary = DenseVectors::new(vec![Fq::from(0)]);
    let mut ivc =
        Ivc::<Bn254Driver, GrumpkinDriver, ExampleFunction<Fr>, ExampleFunction<Fq>>::init(
            &pp,
            z0_primary,
            z0_secondary,
        );

    (0..2).for_each(|_| {
        ivc.prove_step(&pp);
    });
    let proof = ivc.prove_step(&pp);

    new_test_ext().execute_with(|| {
        assert_ok!(SumStorage::set_thing_1(Origin::signed(1), 42, proof, pp));
        assert_eq!(SumStorage::get_sum(), 42);
    });
}
