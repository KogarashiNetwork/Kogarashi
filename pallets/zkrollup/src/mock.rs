use crate as zkrollup_pallet;
use bls_12_381::Fr;
use ec_pairing::TatePairing;
use frame_support::parameter_types;
use frame_system as system;
use jub_jub::JubjubAffine;
use red_jubjub::{PublicKey, RedJubjub};
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};
use zkrollup::{Batch, BatchCircuit, Poseidon, Transaction};
use zkstd::common::Pairing;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Module, Call, Config, Storage, Event<T>},
        Plonk: pallet_plonk::{Module, Call, Storage, Event<T>},
        TemplateModule: zkrollup_pallet::{Module, Call, Storage, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
    type BaseCallFilter = ();
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
}

impl pallet_plonk::Config for Test {
    type Pairing = TatePairing;
    type Affine = JubjubAffine;
    type CustomCircuit = BatchCircuit<RedJubjub, Poseidon<Fr, 2>, 2, 2>;
    type Event = Event;
}

impl zkrollup_pallet::Config for Test {
    type Event = Event;
    type Transaction = Transaction<Self::RedDsa>;
    type PublicKey = PublicKey<Self::RedDsa>;
    type Plonk = Plonk;
    type RedDsa = RedJubjub;
    type Batch = Batch<
        Self::RedDsa,
        Poseidon<<<Self as pallet_plonk::Config>::Pairing as Pairing>::ScalarField, 2>,
        2,
        2,
    >;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
}
