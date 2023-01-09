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
use sp_std::vec::Vec;
use zero_plonk::prelude::Compiler;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The circuit customized by developer
        type CustomCircuit: Circuit;

        /// The overarching event type
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::storage]
    #[pallet::getter(fn public_parameter)]
    /// The setup parameter referred to as SRS
    pub type PublicParameter<T: Config> = StorageValue<_, PublicParameters>;

    #[pallet::event]
    #[pallet::metadata(u32 = "Metadata")]
    pub enum Event<T: Config> {
        /// The event called when setup parameter
        TrustedSetup(PublicParameters),
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
        #[pallet::weight(10_000)]
        pub fn trusted_setup(
            origin: OriginFor<T>,
            val: u32,
            rng: FullcodecRng,
        ) -> DispatchResultWithPostInfo {
            let transactor = ensure_signed(origin)?;
            <Self as Plonk<_>>::trusted_setup(&transactor, val, rng)?;
            Ok(().into())
        }

        /// The function called when we verify the statement
        #[pallet::weight(10_000)]
        pub fn verify(
            origin: OriginFor<T>,
            proof: Proof,
            public_inputs: Vec<Fr>,
        ) -> DispatchResultWithPostInfo {
            let transactor = ensure_signed(origin)?;
            <Self as Plonk<_>>::verify(&transactor, proof, public_inputs)?;
            Ok(().into())
        }
    }
}

impl<T: Config> Pallet<T> {
    /// The RPC method to get public parameters
    pub fn get_public_parameters() -> Option<PublicParameters> {
        PublicParameter::<T>::get()
    }
}

impl<T: Config> Plonk<T::AccountId> for Pallet<T> {
    /// The circuit customized by developer
    type CustomCircuit = T::CustomCircuit;

    /// The API method to setup public parameters
    fn trusted_setup(
        _who: &T::AccountId,
        val: u32,
        mut rng: FullcodecRng,
    ) -> DispatchResultWithPostInfo {
        match Self::public_parameter() {
            Some(_) => {
                return Err(DispatchErrorWithPostInfo {
                    post_info: PostDispatchInfo::from(()),
                    error: DispatchError::Other("already setup"),
                })
            }
            None => {
                let pp = PublicParameters::setup(1 << val, &mut rng).unwrap();
                PublicParameter::<T>::put(&pp);
                Event::<T>::TrustedSetup(pp);
                return Ok(().into());
            }
        }
    }

    /// The API method to verify the proof validity
    fn verify(
        _who: &T::AccountId,
        proof: Proof,
        public_inputs: Vec<Fr>,
    ) -> DispatchResultWithPostInfo {
        match Self::public_parameter() {
            Some(pp) => {
                let label = b"verify";
                let (_, verifier) = Compiler::compile::<T::CustomCircuit>(&pp, label)
                    .expect("failed to compile circuit");
                match verifier.verify(&proof, &public_inputs) {
                    Ok(_) => return Ok(().into()),
                    Err(_) => {
                        return Err(DispatchErrorWithPostInfo {
                            post_info: PostDispatchInfo::from(()),
                            error: DispatchError::Other("invalid proof"),
                        })
                    }
                }
            }
            None => {
                return Err(DispatchErrorWithPostInfo {
                    post_info: PostDispatchInfo::from(()),
                    error: DispatchError::Other("setup not yet"),
                })
            }
        }
    }
}
