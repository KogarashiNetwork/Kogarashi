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

//! # Plonk Pallet
//!
//! - [`Config`]
//! - [`Call`]
//! - [`Pallet`]
//! - [`Plonk`]
//!
//! ## Overview
//!
//! The Plonk pallet provides functions for:
//!
//! - Setup public parameters API
//! - Get public parameters RPC
//! - Verify zkp proof API
//!
//! ### Terminology
//!
//! - **Custom Circuit** The circuit type should be replaced with your own circuit.
//! This circuit should be defined on both blockchain runtime and offchain client.
//!
//! - **Public Parameter** The parameter generated during setup. The users can use
//! this parameter to prove their transaction validity. This parameter can be gotten
//! throught RPC client.
//!
//! ### Intruduce
//! There four steps to use `plonk-pallet`.
//!
//! 1. Import `plonk-pallet` to your substrate runtime and node
//! 2. Define your custom circuit and overwride circuit type
//! 3. Use `plonk-pallet` in your pallet
//! 4. Open `get_public_parameters` RPC
//!
//! `get_public_parameters` is the RPC method and, `trusted_setup` and `verify` are
//! the dispatchable function and API for other pallet.
//!
//! You can see the details with [tutorial](https://astarnetwork.github.io/plonk)
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

pub use pallet::*;

#[cfg(feature = "std")]
#[cfg(test)]
mod mock;

#[cfg(feature = "std")]
#[cfg(test)]
mod tests;

mod traits;
mod types;

pub use traits::Plonk;
pub use types::*;

use frame_support::dispatch::{DispatchErrorWithPostInfo, PostDispatchInfo};
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use zkplonk::prelude::{Plonk as ConstraintPlonk, PlonkKey};
use zksnarks::keypair::Keypair;
use zksnarks::plonk::PlonkParams;
use zksnarks::public_params::PublicParameters as PublicParametersTrait;
use zkstd::common::{Pairing, TwistedEdwardsAffine, Vec};

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The verification pairing domain
        type Pairing: Pairing;

        type Affine: TwistedEdwardsAffine<
            Range = <<Self as pallet::Config>::Pairing as Pairing>::ScalarField,
        >;
        /// The circuit customized by developer
        type CustomCircuit: Circuit<Self::Affine, ConstraintSystem = ConstraintPlonk<Self::Affine>>;

        /// The overarching event type
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::storage]
    #[pallet::getter(fn public_params)]
    /// The setup parameter referred to as SRS
    pub type PublicParameters<T: Config> = StorageValue<_, PlonkParams<T::Pairing>>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    #[pallet::metadata(u32 = "Metadata")]
    pub enum Event<T: Config> {
        /// The event called when setup parameter
        TrustedSetup(PlonkParams<T::Pairing>),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Error names should be descriptive
        NoneValue,
        /// Errors should have helpful documentation associated with them
        StorageOverflow,
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// The function called when we setup the parameters
        // SBP-M1 review: missing proper benchmarking
        #[pallet::weight(10_000)]
        pub fn trusted_setup(
            origin: OriginFor<T>,
            val: u32,
            rng: FullcodecRng,
        ) -> DispatchResultWithPostInfo {
            ensure_signed(origin)?;
            <Self as Plonk<T::Pairing, T::Affine>>::trusted_setup(val, rng)?;
            Ok(().into())
        }

        /// The function called when we verify the statement
        // SBP-M1 review: missing proper benchmarking
        #[pallet::weight(10_000)]
        pub fn verify(
            origin: OriginFor<T>,
            proof: Proof<T::Pairing>,
            // SBP-M1 review: use BoundedVec instead of Vec
            public_inputs: Vec<<T::Pairing as Pairing>::ScalarField>,
        ) -> DispatchResultWithPostInfo {
            ensure_signed(origin)?;
            <Self as Plonk<T::Pairing, T::Affine>>::verify(proof, public_inputs)?;
            Ok(().into())
        }
    }
}

impl<T: Config> Pallet<T> {
    /// The RPC method to get public parameters
    pub fn get_public_params() -> Option<PlonkParams<T::Pairing>> {
        PublicParameters::<T>::get()
    }
}

impl<T: Config> Plonk<T::Pairing, T::Affine> for Pallet<T> {
    /// The circuit customized by developer
    type CustomCircuit = T::CustomCircuit;

    /// The API method to setup public parameters
    fn trusted_setup(
        // SBP-M1 review: why do you pass unused parameter?
        // Remove if redundant
        val: u32,
        mut rng: FullcodecRng,
    ) -> DispatchResultWithPostInfo {
        match Self::public_params() {
            Some(_) => Err(DispatchErrorWithPostInfo {
                post_info: PostDispatchInfo::from(()),
                // SBP-M1 review: define proper error type
                error: DispatchError::Other("already setup"),
            }),
            None => {
                let pp = PlonkParams::<T::Pairing>::setup(val as u64, &mut rng);
                PublicParameters::<T>::put(&pp);
                Self::deposit_event(Event::<T>::TrustedSetup(pp));
                Ok(().into())
            }
        }
    }

    /// The API method to verify the proof validity
    fn verify(
        // SBP-M1 review: why do you pass unused parameter?
        // Remove if redundant
        proof: Proof<T::Pairing>,
        public_inputs: Vec<<T::Pairing as Pairing>::ScalarField>,
    ) -> DispatchResultWithPostInfo {
        match Self::public_params() {
            Some(pp) => {
                let (_, verifier) =
                    PlonkKey::<T::Pairing, T::Affine, T::CustomCircuit>::compile(&pp)
                        // SBP-M1 review: use proper error handling in extrinsics' code instead of `expect`/`unwrap`
                        .expect("failed to compile circuit");
                match verifier.verify(&proof, &public_inputs) {
                    Ok(_) => Ok(().into()),
                    Err(_) => Err(DispatchErrorWithPostInfo {
                        post_info: PostDispatchInfo::from(()),
                        // SBP-M1 review: define proper error type
                        error: DispatchError::Other("invalid proof"),
                    }),
                }
            }
            None => Err(DispatchErrorWithPostInfo {
                post_info: PostDispatchInfo::from(()),
                // SBP-M1 review: define proper error type
                error: DispatchError::Other("setup not yet"),
            }),
        }
    }
}
