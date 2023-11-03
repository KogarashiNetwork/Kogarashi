// Copyright (C) 2023-2024 Inverse (JP) LLC.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]
#![allow(unused_variables)]

pub use pallet::*;

#[cfg(feature = "std")]
#[cfg(test)]
mod mock;

#[cfg(feature = "std")]
#[cfg(test)]
mod tests;

mod traits;

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use pallet_plonk::{FullcodecRng, Plonk, Proof};
use traits::Rollup;
use zkstd::common::{Pairing, RedDSA};

#[frame_support::pallet]
pub mod pallet {

    use super::*;
    use zkrollup::BatchGetter;
    use zkstd::common::Pairing;

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_plonk::Config {
        type Plonk: Plonk<
            <Self as pallet_plonk::Config>::Pairing,
            <<Self as pallet::Config>::RedDsa as RedDSA>::Affine,
        >;
        type RedDsa: RedDSA<
            Range = <<Self as pallet_plonk::Config>::Pairing as Pairing>::ScalarField,
        >;
        type Transaction: Parameter + Member + Default + Copy;
        type Batch: BatchGetter<<Self as pallet::Config>::RedDsa>
            + Parameter
            + Member
            + Default
            + Clone;
        type PublicKey: Parameter + Member + Default + Copy;

        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::storage]
    #[pallet::getter(fn state_root)]
    /// The setup parameter referred to as SRS
    pub type StateRoot<T: Config> =
        StorageValue<_, <<T as pallet_plonk::Config>::Pairing as Pairing>::ScalarField>;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // < HB SBP M2 review:
        //
        // Missing weight from benchmarking and inline code from extrinsic
        //
        // >
        #[pallet::weight(10_000)]
        pub fn trusted_setup(
            origin: OriginFor<T>,
            // < HB SBP M2 review
            //
            // Why don't declaring a Degree type?
            //
            // >
            degree: u32,
            rng: pallet_plonk::FullcodecRng,
        ) -> DispatchResultWithPostInfo {
            pallet_plonk::Pallet::<T>::trusted_setup(origin, degree, rng)?;
            Ok(().into())
        }

        // < HB SBP M2 review:
        //
        // Missing weight from benchmarking and inline code from extrinsic
        //
        // >
        #[pallet::weight(10_000)]
        pub(super) fn deposit(
            origin: OriginFor<T>,
            amount: u64,
            address: T::PublicKey,
        ) -> DispatchResultWithPostInfo {
            ensure_signed(origin)?;

            Self::deposit_event(Event::Deposit(amount, address));
            Ok(().into())
        }

        // < HB SBP M2 review:
        //
        // Missing weight from benchmarking and inline code from extrinsic
        //
        // >
        #[pallet::weight(10_000)]
        pub(super) fn withdraw(
            origin: OriginFor<T>,
            // l2_burn_merkle_proof: MerkleProof<F, H, N>,
            batch_root: <<T as pallet_plonk::Config>::Pairing as Pairing>::ScalarField,
            transaction: T::Transaction,
            l1_address: T::PublicKey,
        ) -> DispatchResultWithPostInfo {
            ensure_signed(origin)?;
            // merkle_verify(l2_burn_merkle_proof, batch_root);
            // send(transaction.amount, l1_address);
            Ok(().into())
        }

        // < HB SBP M2 review:
        //
        // Missing weight from benchmarking and inline code from extrinsic
        //
        // >
        #[pallet::weight(10_000)]
        pub fn set_initial_root(
            origin: OriginFor<T>,
            root: <<T as pallet_plonk::Config>::Pairing as Pairing>::ScalarField,
        ) -> DispatchResultWithPostInfo {
            // Need to ensure that the caller is operator
            ensure_signed(origin)?;

            StateRoot::<T>::put(root);
            Self::deposit_event(Event::StateInitialized(root));
            Ok(().into())
        }

        // < HB SBP M2 review:
        //
        // Missing weight from benchmarking and inline code from extrinsic
        //
        // >
        #[pallet::weight(10_000)]
        pub fn update_state(
            origin: OriginFor<T>,
            proof: Proof<<T as pallet_plonk::Config>::Pairing>,
            public_inputs: Vec<<<T as pallet_plonk::Config>::Pairing as Pairing>::ScalarField>,
            compressed_batch_data: T::Batch,
        ) -> DispatchResultWithPostInfo {
            ensure_signed(origin)?;

            T::Plonk::verify(proof, public_inputs)?;

            let new_root = compressed_batch_data.final_root();

            for (amount, address) in compressed_batch_data.withdraw_info() {
                // process withdrawals
            }

            StateRoot::<T>::put(new_root);
            Self::deposit_event(Event::StateUpdated(new_root));
            Ok(().into())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    #[pallet::metadata(T::Transaction = "Transaction")]
    pub enum Event<T: Config> {
        /// Deposit to process on L2
        Deposit(u64, <T as Config>::PublicKey),
        /// State update after proof verification
        StateUpdated(<<T as pallet_plonk::Config>::Pairing as Pairing>::ScalarField),
        /// State update after proof verification
        StateInitialized(<<T as pallet_plonk::Config>::Pairing as Pairing>::ScalarField),
    }
}

impl<T: Config> Rollup for Pallet<T> {
    type F = <<T as pallet_plonk::Config>::Pairing as Pairing>::ScalarField;
    type Transaction = T::Transaction;
    type Batch = T::Batch;
    type PublicKey = T::PublicKey;

    fn state_root() -> Self::F {
        Self::state_root().expect("No state root")
    }

    fn trusted_setup(val: u32, rng: FullcodecRng) -> DispatchResultWithPostInfo {
        T::Plonk::trusted_setup(val, rng)
    }
}
