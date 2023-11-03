use crate::{
    self as confidential_transfer,
    circuit::{ConfidentialTransferCircuit, ConfidentialTransferTransaction},
    pallet::Config,
};
use jub_jub::{Fp as JubJubScalar, JubjubAffine, JubjubExtended};
use pallet_encrypted_balance::{Account, AccountData};
use she_elgamal::EncryptedNumber;
use zkstd::common::TwistedEdwardsCurve;

use ec_pairing::TatePairing;
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
        EncryptedCurrency: pallet_encrypted_balance::{Module, Call, Storage, Config<T>, Event<T>},
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
    type Pairing = TatePairing;
    type Affine = JubjubAffine;
    type CustomCircuit = ConfidentialTransferCircuit;
    type Event = Event;
}

impl pallet_encrypted_balance::Config for TestRuntime {
    type Affine = JubjubAffine;
    type EncryptedBalance = EncryptedNumber;
    type Event = Event;
    type AccountStore = StorageMapShim<
        Account<TestRuntime>,
        frame_system::Provider<TestRuntime>,
        u64,
        AccountData<Self::EncryptedBalance>,
    >;
    type WeightInfo = ();
}

impl Config for TestRuntime {
    type Plonk = Plonk;
    type EncryptedCurrency = EncryptedCurrency;
    type Event = Event;
}

// confidential transfer test data
pub(crate) const ALICE_ADDRESS: u64 = 1;
pub(crate) const BOB_ADDRESS: u64 = 2;
pub(crate) const ALICE_PRIVATE_KEY: JubJubScalar = JubJubScalar::to_mont_form([1, 0, 0, 0]);
pub(crate) const BOB_PRIVATE_KEY: JubJubScalar = JubJubScalar::to_mont_form([2, 0, 0, 0]);
const ALICE_RANDOMNESS: JubJubScalar = JubJubScalar::to_mont_form([1, 2, 3, 4]);
const BOB_RANDOMNESS: JubJubScalar = JubJubScalar::to_mont_form([4, 3, 2, 1]);
const TRANFER_RANDOMNESS: JubJubScalar = JubJubScalar::to_mont_form([5, 6, 7, 8]);
pub(crate) const ALICE_BALANCE: u32 = 1500;
pub(crate) const BOB_BALANCE: u32 = 0;
pub(crate) const TRANSFER_AMOUNT: u32 = 800;
pub(crate) const ALICE_AFTER_BALANCE: u32 = ALICE_BALANCE - TRANSFER_AMOUNT;
pub(crate) const BOB_AFTER_BALANCE: u32 = BOB_BALANCE + TRANSFER_AMOUNT;

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub(crate) fn new_test_ext() -> sp_io::TestExternalities {
    let alice_balance =
        EncryptedNumber::encrypt(ALICE_PRIVATE_KEY, ALICE_BALANCE, ALICE_RANDOMNESS);
    let bob_balance = EncryptedNumber::encrypt(BOB_PRIVATE_KEY, BOB_BALANCE, BOB_RANDOMNESS);

    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<TestRuntime>()
        .unwrap();
    pallet_encrypted_balance::GenesisConfig::<TestRuntime> {
        balances: vec![(ALICE_ADDRESS, alice_balance), (BOB_ADDRESS, bob_balance)],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| System::set_block_number(1));
    ext
}

pub(crate) fn generate_confidential_transfer_params() -> (
    ConfidentialTransferCircuit,
    ConfidentialTransferTransaction<EncryptedNumber, JubjubAffine>,
) {
    let alice_public_key = JubjubExtended::ADDITIVE_GENERATOR * ALICE_PRIVATE_KEY;
    let bob_public_key = JubjubExtended::ADDITIVE_GENERATOR * BOB_PRIVATE_KEY;
    let transfer_amount = JubJubScalar::from(TRANSFER_AMOUNT as u64);
    let alice_after_balance = JubJubScalar::from(ALICE_AFTER_BALANCE as u64);

    let alice_balance =
        EncryptedNumber::encrypt(ALICE_PRIVATE_KEY, ALICE_BALANCE, ALICE_RANDOMNESS);
    let alice_transfer_amount =
        EncryptedNumber::encrypt(ALICE_PRIVATE_KEY, TRANSFER_AMOUNT, TRANFER_RANDOMNESS);
    let bob_encrypted_transfer_amount = (JubjubExtended::ADDITIVE_GENERATOR * transfer_amount)
        + (bob_public_key * TRANFER_RANDOMNESS);
    let alice_public_key = JubjubAffine::from(alice_public_key);
    let bob_public_key = JubjubAffine::from(bob_public_key);
    let bob_encrypted_transfer_amount = JubjubAffine::from(bob_encrypted_transfer_amount);
    let bob_encrypted_transfer_amount_other =
        (JubjubExtended::ADDITIVE_GENERATOR * TRANFER_RANDOMNESS).into();

    (
        ConfidentialTransferCircuit::new(
            alice_public_key,
            bob_public_key,
            alice_balance,
            alice_transfer_amount,
            bob_encrypted_transfer_amount,
            ALICE_PRIVATE_KEY,
            transfer_amount,
            alice_after_balance,
            TRANFER_RANDOMNESS,
        ),
        ConfidentialTransferTransaction::new(
            alice_public_key,
            bob_public_key,
            alice_transfer_amount,
            bob_encrypted_transfer_amount,
            bob_encrypted_transfer_amount_other,
        ),
    )
}
