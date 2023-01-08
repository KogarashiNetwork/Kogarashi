#[allow(dead_code)]
use crate as plonk_pallet;
use crate::*;
use frame_support::parameter_types;
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};

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
        TemplateModule: plonk_pallet::{Module, Call, Storage, Event<T>},
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
use rand::rngs::StdRng;
use rand::SeedableRng;
use zero_crypto::behave::Group;
use zero_plonk::prelude::{Error as CircuitError, *};

pub struct DummyCircuit {
    a: JubJubScalar,
    b: JubJubExtended,
}

impl DummyCircuit {
    pub fn new(a: JubJubScalar) -> Self {
        Self {
            a,
            b: zero_jubjub::GENERATOR_EXTENDED * &a,
        }
    }
}

impl Default for DummyCircuit {
    fn default() -> Self {
        Self::new(JubJubScalar::from(7u64))
    }
}

impl Circuit for DummyCircuit {
    fn circuit<C>(&self, composer: &mut C) -> Result<(), CircuitError>
    where
        C: Composer,
    {
        let w_a = composer.append_witness(self.a);
        let w_b = composer.append_point(self.b);

        let w_x = composer.component_mul_generator(w_a, zero_jubjub::GENERATOR_EXTENDED)?;

        composer.assert_equal_point(w_b, w_x);

        Ok(())
    }
}

impl plonk_pallet::Config for Test {
    type Event = Event;
    type CustomCircuit = DummyCircuit;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
}
