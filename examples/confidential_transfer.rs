use confidential_transfer::{
    ConfidentialTransfer as OtherConfidentialTransfer, ConfidentialTransferCircuit,
    ConfidentialTransferTransaction,
};
use ec_pairing::TatePairing;
use frame_support::traits::StorageMapShim;
use frame_support::{assert_ok, construct_runtime, parameter_types};
use jub_jub::{Fp, JubjubAffine, JubjubExtended};
use pallet_encrypted_balance::{Account, AccountData};
use pallet_plonk::FullcodecRng;
use rand::Rng;
use rand_core::{OsRng, SeedableRng};
use she_elgamal::EncryptedNumber;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};
use zkplonk::prelude::PlonkKey;
use zksnarks::keypair::Keypair;
use zkstd::common::{Group, TwistedEdwardsCurve};

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

impl confidential_transfer::Config for TestRuntime {
    type Plonk = Plonk;
    type EncryptedCurrency = EncryptedCurrency;
    type Event = Event;
}

#[allow(clippy::too_many_arguments)]
fn new_test_ext(
    alice_address: u64,
    alice_private_key: Fp,
    alice_balance: u16,
    alice_radomness: Fp,
    bob_private_key: Fp,
    bob_address: u64,
    bob_balance: u16,
    bob_radomness: Fp,
) -> sp_io::TestExternalities {
    let alice_balance =
        EncryptedNumber::encrypt(alice_private_key, alice_balance.into(), alice_radomness);
    let bob_balance = EncryptedNumber::encrypt(bob_private_key, bob_balance.into(), bob_radomness);

    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<TestRuntime>()
        .unwrap();
    pallet_encrypted_balance::GenesisConfig::<TestRuntime> {
        balances: vec![(alice_address, alice_balance), (bob_address, bob_balance)],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| System::set_block_number(1));
    ext
}

fn generate_default_test_data() -> (u64, Fp, u16, Fp, Fp, u64, u16, Fp, u16, u16, u16, Fp) {
    let mut rng = rand::thread_rng();

    let alice_address = rng.gen::<u64>();
    let alice_private_key = Fp::random(OsRng);
    let alice_balance = rng.gen::<u16>();
    let alice_radomness = Fp::random(OsRng);
    let bob_private_key = Fp::random(OsRng);
    let bob_address = rng.gen::<u64>();
    let bob_balance = rng.gen::<u16>();
    let bob_radomness = Fp::random(OsRng);
    let transfer_amount = rng.gen_range(0..alice_balance);
    let alice_after_balance = alice_balance - transfer_amount;
    let bob_after_balance = bob_balance + transfer_amount;
    let transfer_randomness = Fp::random(OsRng);

    (
        alice_address,
        alice_private_key,
        alice_balance,
        alice_radomness,
        bob_private_key,
        bob_address,
        bob_balance,
        bob_radomness,
        transfer_amount,
        alice_after_balance,
        bob_after_balance,
        transfer_randomness,
    )
}

fn get_rng() -> FullcodecRng {
    FullcodecRng::from_seed([
        0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06, 0xbc,
        0xe5,
    ])
}

fn main() {
    let k = 14;
    let mut rng = get_rng();
    let (
        alice_address,
        alice_private_key,
        alice_balance,
        alice_radomness,
        bob_private_key,
        bob_address,
        bob_balance,
        bob_radomness,
        transfer_amount,
        alice_after_balance,
        bob_after_balance,
        transfer_randomness,
    ) = generate_default_test_data();
    new_test_ext(
        alice_address,
        alice_private_key,
        alice_balance,
        alice_radomness,
        bob_private_key,
        bob_address,
        bob_balance,
        bob_radomness,
    )
    .execute_with(|| {
        // default balance decryption check
        let alice_encrypted_balance = ConfidentialTransfer::total_balance(&alice_address);
        let alice_raw_balance = alice_encrypted_balance.decrypt(alice_private_key);
        let bob_encrypted_balance = ConfidentialTransfer::total_balance(&bob_address);
        let bob_raw_balance = bob_encrypted_balance.decrypt(bob_private_key);

        assert_eq!(alice_raw_balance.unwrap() as u16, alice_balance);
        assert_eq!(bob_raw_balance.unwrap() as u16, bob_balance);

        // trusted setup check
        let result =
            ConfidentialTransfer::trusted_setup(Origin::signed(alice_address), k, rng.clone());
        assert_ok!(result);

        // proof generation
        let pp = Plonk::public_params().unwrap();
        let alice_public_key = JubjubExtended::ADDITIVE_GENERATOR * alice_private_key;
        let bob_public_key = JubjubExtended::ADDITIVE_GENERATOR * bob_private_key;
        let transfer_amount_scalar = Fp::from(transfer_amount as u64);
        let alice_after_balance_scalar = Fp::from(alice_after_balance as u64);

        let alice_balance =
            EncryptedNumber::encrypt(alice_private_key, alice_balance.into(), alice_radomness);
        let alice_transfer_amount = EncryptedNumber::encrypt(
            alice_private_key,
            transfer_amount.into(),
            transfer_randomness,
        );
        let bob_encrypted_transfer_amount = (JubjubExtended::ADDITIVE_GENERATOR
            * transfer_amount_scalar)
            + (bob_public_key * transfer_randomness);
        let alice_public_key = JubjubAffine::from(alice_public_key);
        let bob_public_key = JubjubAffine::from(bob_public_key);
        let bob_encrypted_transfer_amount = JubjubAffine::from(bob_encrypted_transfer_amount);
        let bob_encrypted_transfer_amount_other =
            (JubjubExtended::ADDITIVE_GENERATOR * transfer_randomness).into();

        let confidential_transfer_circuit = ConfidentialTransferCircuit::new(
            alice_public_key,
            bob_public_key,
            alice_balance,
            alice_transfer_amount,
            bob_encrypted_transfer_amount,
            alice_private_key,
            transfer_amount_scalar,
            alice_after_balance_scalar,
            transfer_randomness,
        );
        let prover =
            PlonkKey::<TatePairing, JubjubAffine, ConfidentialTransferCircuit>::compile(&pp)
                .expect("failed to compile circuit");
        let proof = prover
            .0
            .create_proof(&mut rng, &confidential_transfer_circuit)
            .expect("failed to prove");

        // confidential transfer check
        let transaction_params = ConfidentialTransferTransaction::new(
            alice_public_key,
            bob_public_key,
            alice_transfer_amount,
            bob_encrypted_transfer_amount,
            bob_encrypted_transfer_amount_other,
        );
        let result = ConfidentialTransfer::confidential_transfer(
            Origin::signed(alice_address),
            bob_address,
            proof.0,
            transaction_params,
        );
        assert_ok!(result);

        // balance transition check
        let alice_balance = ConfidentialTransfer::total_balance(&alice_address);
        let alice_raw_balance = alice_balance.decrypt(alice_private_key);
        let bob_balance = ConfidentialTransfer::total_balance(&bob_address);
        let bob_raw_balance = bob_balance.decrypt(bob_private_key);

        assert_eq!(alice_raw_balance.unwrap() as u16, alice_after_balance);
        assert_eq!(bob_raw_balance.unwrap() as u16, bob_after_balance);
    })
}
