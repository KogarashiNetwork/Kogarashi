use crate::{self as confidential_transfer, pallet::Config};

use zero_circuits::ConfidentialTransferCircuit;
use zero_elgamal::EncryptedNumber;

use frame_support::traits::StorageMapShim;
use frame_support::{construct_runtime, parameter_types};
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
        Plonk: pallet_plonk::{Module, Call, Storage, Event<T>},
        EncryptedBalance: pallet_encrypted_balance::{Module, Call, Storage, Config<T>, Event<T>},
        ConfidentialTransfer: confidential_transfer::{Module, Call, Storage, Event<T>},
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
    type CustomCircuit = ConfidentialTransferCircuit;
    type Event = Event;
}

impl pallet_encrypted_balance::Config for TestRuntime {
    type EncryptedBalance = EncryptedNumber;
    type Event = Event;
    type AccountStore = StorageMapShim<
        super::Account<TestRuntime>,
        frame_system::Provider<TestRuntime>,
        u64,
        super::AccountData<Self::EncryptedBalance>,
    >;
    type WeightInfo = ();
}
impl Config for TestRuntime {
    type Event = Event;
}

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub(crate) fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::default()
        .build_storage::<TestRuntime>()
        .unwrap()
        .into()
}
