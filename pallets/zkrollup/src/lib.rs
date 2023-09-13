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

#[frame_support::pallet]
pub mod pallet {

    use super::*;

    use zkrollup::BatchGetter;
    use zkstd::common::{FftField, Pairing};

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_plonk::Config {
        type Plonk: Plonk<<Self as pallet_plonk::Config>::P>;
        type F: FftField + Parameter + Member + Default + Copy;
        type Transaction: Parameter + Member + Default + Copy;
        type Batch: BatchGetter<Self::F> + Parameter + Member + Default + Clone;
        type PublicKey: Parameter + Member + Default + Copy;

        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::storage]
    #[pallet::getter(fn state_root)]
    /// The setup parameter referred to as SRS
    pub type StateRoot<T: Config> = StorageValue<_, T::F>;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn trusted_setup(
            origin: OriginFor<T>,
            degree: u32,
            rng: pallet_plonk::FullcodecRng,
        ) -> DispatchResultWithPostInfo {
            pallet_plonk::Pallet::<T>::trusted_setup(origin, degree, rng)?;
            Ok(().into())
        }

        #[pallet::weight(10_000)]
        pub(super) fn deposit(
            origin: OriginFor<T>,
            t: T::Transaction,
        ) -> DispatchResultWithPostInfo {
            ensure_signed(origin)?;

            Self::deposit_event(Event::Deposit(t));
            Ok(().into())
        }

        #[pallet::weight(10_000)]
        pub(super) fn withdraw(
            origin: OriginFor<T>,
            // l2_burn_merkle_proof: MerkleProof<F, H, N>,
            batch_root: T::F,
            transaction: T::Transaction,
            l1_address: T::PublicKey,
        ) -> DispatchResultWithPostInfo {
            ensure_signed(origin)?;
            // merkle_verify(l2_burn_merkle_proof, batch_root);
            // send(transaction.amount, l1_address);
            Ok(().into())
        }

        #[pallet::weight(10_000)]
        pub fn set_initial_root(origin: OriginFor<T>, root: T::F) -> DispatchResultWithPostInfo {
            // Need to ensure that the caller is operator
            ensure_signed(origin)?;

            StateRoot::<T>::put(root);
            Self::deposit_event(Event::StateInitialized(root));
            Ok(().into())
        }

        #[pallet::weight(10_000)]
        pub fn update_state(
            origin: OriginFor<T>,
            proof: Proof<<T as pallet_plonk::Config>::P>,
            public_inputs: Vec<<<T as pallet_plonk::Config>::P as Pairing>::ScalarField>,
            compressed_batch_data: T::Batch,
        ) -> DispatchResultWithPostInfo {
            ensure_signed(origin)?;

            T::Plonk::verify(proof, public_inputs)?;

            let new_root = compressed_batch_data.final_root();

            StateRoot::<T>::put(new_root);
            Self::deposit_event(Event::StateUpdated(new_root));
            Ok(().into())
        }

        // pub fn check_balance(&self, merkle_proof: MerkleProof<F, H, N>) -> u64 {
        //     // merkle_verify(merkle_proof, self.rollup_state_root);
        //     // get_balance()
        //     0
        // }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    #[pallet::metadata(T::Transaction = "Transaction")]
    pub enum Event<T: Config> {
        /// Deposit to process on L2
        Deposit(T::Transaction),
        /// State update after proof verification
        StateUpdated(T::F),
        /// State update after proof verification
        StateInitialized(T::F),
    }
}

impl<T: Config> Rollup for Pallet<T> {
    type F = T::F;
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
