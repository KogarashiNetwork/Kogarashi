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

pub use pallet::*;

use frame_support::dispatch::{DispatchErrorWithPostInfo, PostDispatchInfo};
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use zkstd::behave::Group;
use zkstd::common::{Pairing, Vec};

#[frame_support::pallet]
pub mod pallet {

    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use pallet_zk_rollup::FieldHasher;
    use zkstd::common::{FftField, Pairing};

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type F: FftField;
        type H: FieldHasher<Self::F, 2>;
        type Transaction: Parameter + Member + Default + Copy + MaybeSerializeDeserialize;
        type Batch: Parameter + Member + Default + Copy + MaybeSerializeDeserialize;
        type Proof;

        type BatchSize: Get<u32>;

        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::storage]
    #[pallet::getter(fn state_root)]
    /// The setup parameter referred to as SRS
    pub type StateRoot<T: Config> = StorageValue<_, T::F>;

    #[pallet::storage]
    #[pallet::getter(fn deposits)]
    pub(super) type Deposits<T: Config> = StorageValue<_, Vec<T::Transaction>>;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T>
    // where
    //     pallet::Call<T, I>: Codec,
    {
        #[pallet::weight(10_000)]
        pub(super) fn deposit(
            origin: OriginFor<T>,
            t: T::Transaction,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            let mut deposits = Deposits::<T>::get().unwrap();
            deposits.push(t);
            if deposits.len() == T::BatchSize::get() {
                Self::deposit_event(Event::DepositsReady(sp_std::mem::take(&mut deposits)));
            }
            Ok(().into())
        }

        #[pallet::weight(10_000)]
        pub(super) fn withdraw(
            origin: OriginFor<T>,
            // l2_burn_merkle_proof: MerkleProof<F, H, N>,
            batch_root: Self::F,
            transaction: Transaction,
            l1_address: PublicKey,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            // merkle_verify(l2_burn_merkle_proof, batch_root);
            // send(transaction.amount, l1_address);
            Ok(().into())
        }

        // fn update_state(&mut self, new_state_root: F) {
        //     self.rollup_state_root = new_state_root;
        // }

        #[pallet::weight(10_000)]
        pub fn add_batch(
            origin: OriginFor<T>,
            proof: Proof<T::F, T::H, N, BATCH_SIZE>,
            compressed_batch_data: Batch<F, H, N, BATCH_SIZE>,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            // assert!(self.verifier_contract.verify_proof(proof));

            StateRoot::<T>::put(&compressed_batch_data.final_root());
            self.update_state(compressed_batch_data.final_root());
            self.calldata.push(compressed_batch_data);
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
        /// Required amount of deposits are ready to be processed on L2
        DepositsReady(Vec<T::Transaction>),
        StateUpdated(T::F),
    }
}

impl<T: Config> Pallet<T> {}
