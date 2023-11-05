#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

use ec_pairing::TatePairing;
use jub_jub::{Fp as JubJubScalar, JubjubAffine, JubjubExtended};
use pallet::*;
use pallet_plonk::{BlsScalar, Circuit, FullcodecRng, Proof};
use zkplonk::{prelude::*, Plonk as PlonkConstraint};
use zksnarks::keypair::Keypair;
use zkstd::common::{Pairing, TwistedEdwardsCurve};

use frame_support::{assert_ok, construct_runtime, parameter_types};
use rand_core::SeedableRng;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    /// Copuliing configuration trait with pallet_plonk.
    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_plonk::Config {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    #[pallet::metadata(u32 = "Metadata")]
    pub enum Event<T: Config> {
        ValueSet(u32, u32),
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn thing1)]
    pub type Thing1<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn thing2)]
    pub type Thing2<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // The module's dispatchable functions.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // Coupled trusted setup
        #[pallet::weight(10_000)]
        pub fn trusted_setup(
            origin: OriginFor<T>,
            val: u32,
            rng: FullcodecRng,
        ) -> DispatchResultWithPostInfo {
            pallet_plonk::Pallet::<T>::trusted_setup(origin, val, rng)?;
            Ok(().into())
        }

        /// Sets the first simple storage value
        #[pallet::weight(10_000)]
        pub fn set_thing_1(
            origin: OriginFor<T>,
            val: u32,
            proof: Proof<T::Pairing>,
            public_inputs: Vec<<T::Pairing as Pairing>::ScalarField>,
        ) -> DispatchResultWithPostInfo {
            // Define the proof verification
            pallet_plonk::Pallet::<T>::verify(origin, proof, public_inputs)?;

            Thing1::<T>::put(val);

            Self::deposit_event(Event::ValueSet(1, val));
            Ok(().into())
        }

        /// Sets the second stored value
        #[pallet::weight(10_000)]
        pub fn set_thing_2(origin: OriginFor<T>, val: u32) -> DispatchResultWithPostInfo {
            let _ = ensure_signed(origin)?;

            Thing2::<T>::put(val);

            Self::deposit_event(Event::ValueSet(2, val));
            Ok(().into())
        }
    }
}

impl<T: Config> Pallet<T> {
    pub fn get_sum() -> u32 {
        Thing1::<T>::get() + Thing2::<T>::get()
    }
}

// Implement a circuit that checks:
// 1) a + b = c where C is a PI
// 2) a <= 2^6
// 3) b <= 2^5
// 4) a * b = d where D is a PI
// 5) JubJub::GENERATOR * e(JubJubScalar) = f where F is a Public Input

#[derive(Debug, Default)]
pub struct TestCircuit {
    pub a: BlsScalar,
    pub b: BlsScalar,
    pub c: BlsScalar,
    pub d: BlsScalar,
    pub e: JubJubScalar,
    pub f: JubjubAffine,
}

impl Circuit<JubjubAffine> for TestCircuit {
    type ConstraintSystem = PlonkConstraint<JubjubAffine>;
    fn synthesize(&self, composer: &mut PlonkConstraint<JubjubAffine>) -> Result<(), Error> {
        let a = composer.append_witness(self.a);
        let b = composer.append_witness(self.b);

        // Make first constraint a + b = c
        let constraint = Constraint::default()
            .left(1)
            .right(1)
            .public(-self.c)
            .a(a)
            .b(b);

        composer.append_gate(constraint);

        // Check that a and b are in range
        composer.component_range(a, 1 << 6);
        composer.component_range(b, 1 << 5);

        // Make second constraint a * b = d
        let constraint = Constraint::default()
            .mult(1)
            .output(1)
            .public(-self.d)
            .a(a)
            .b(b);

        composer.append_gate(constraint);

        let e = composer.append_witness(self.e);
        let scalar_mul_result =
            composer.component_mul_generator(e, JubjubExtended::ADDITIVE_GENERATOR)?;
        composer.assert_equal_public_point(scalar_mul_result, self.f);
        Ok(())
    }
}

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;
type Block = frame_system::mocking::MockBlock<TestRuntime>;

construct_runtime!(
    pub enum TestRuntime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Module, Call, Config, Storage, Event<T>},
        // Define the `pallet_plonk` in `contruct_runtime`
        Plonk: pallet_plonk::{Module, Call, Storage, Event<T>},
        SumStorage: pallet::{Module, Call, Storage, Event<T>},
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
    type CustomCircuit = TestCircuit;
    type Event = Event;
}

impl Config for TestRuntime {
    type Event = Event;
}

fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::default()
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

fn main() {
    let mut rng = get_rng();
    let test_circuit = TestCircuit {
        a: BlsScalar::from(20u64),
        b: BlsScalar::from(5u64),
        c: BlsScalar::from(25u64),
        d: BlsScalar::from(100u64),
        e: JubJubScalar::from(2u64),
        f: JubjubAffine::from(JubjubExtended::ADDITIVE_GENERATOR * JubJubScalar::from(2u64)),
    };

    new_test_ext().execute_with(|| {
        assert_eq!(SumStorage::get_sum(), 0);
        assert_ok!(Plonk::trusted_setup(Origin::signed(1), 12, rng.clone()));

        let pp = Plonk::public_params().unwrap();
        let (prover, _) = PlonkKey::<TatePairing, JubjubAffine, TestCircuit>::compile(&pp)
            .expect("failed to compile circuit");

        let (proof, public_inputs) = prover
            .create_proof(&mut rng, &test_circuit)
            .expect("failed to prove");

        assert_ok!(SumStorage::set_thing_1(
            Origin::signed(1),
            42,
            proof,
            public_inputs
        ));
        assert_eq!(SumStorage::get_sum(), 42);
    })
}
