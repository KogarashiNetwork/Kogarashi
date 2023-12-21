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

//! # Nova IVC Pallet
//!
//! - [`Config`]
//! - [`Call`]
//! - [`Pallet`]
//! - [`Ivc`]
//!
//! ## Overview
//!
//! The Nova IVC pallet provides functions for:
//!
//! - Setup public parameters API
//! - Get public parameters RPC
//! - Verify IVC proof API
//!
//! ### Terminology
//!
//! - **Public Parameter** The parameter generated during setup. The users can use
//! this parameter to prove their transaction validity. This parameter can be gotten
//! through RPC client.
//!
//! ### Introduction
//! There four steps to use `nova-pallet`.
//!
//! 1. Import `nova-ivc-pallet` to your substrate runtime and node
//! 2. Use `nova-ivc-pallet` in your pallet
//! 3. Open `get_public_parameters` RPC
//!
//! `get_public_parameters` is an RPC method and, `trusted_setup` and `verify` are
//! the dispatchable function and API for other pallet.
//!
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]

#[cfg(feature = "std")]
#[cfg(test)]
mod mock;
#[cfg(feature = "std")]
#[cfg(test)]
mod tests;
mod traits;

pub use pallet::*;
pub use rand_xorshift::XorShiftRng as FullcodecRng;
pub use traits::Ivc;

use frame_support::dispatch::{DispatchErrorWithPostInfo, PostDispatchInfo};
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use zknova::{PublicParams, RecursiveProof};

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use zknova::{FunctionCircuit, PublicParams, RecursiveProof};
    use zkstd::circuit::CircuitDriver;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type E1: CircuitDriver<Base = <Self::E2 as CircuitDriver>::Scalar>;
        type E2: CircuitDriver<Base = <Self::E1 as CircuitDriver>::Scalar>;
        type FC1: FunctionCircuit<<Self::E1 as CircuitDriver>::Scalar>;
        type FC2: FunctionCircuit<<Self::E2 as CircuitDriver>::Scalar>;
        /// The overarching event type
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::storage]
    #[pallet::getter(fn public_params)]
    pub type PublicParameters<T: Config> =
        StorageValue<_, PublicParams<T::E1, T::E2, T::FC1, T::FC2>>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    #[pallet::metadata(u32 = "Metadata")]
    pub enum Event<T: Config> {
        /// The event called when setup parameter
        TrustedSetup(PublicParams<T::E1, T::E2, T::FC1, T::FC2>),
    }

    #[pallet::error]
    pub enum Error<T> {
        NoneValue,
        StorageOverflow,
    }

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
            rng: FullcodecRng,
        ) -> DispatchResultWithPostInfo {
            ensure_signed(origin)?;
            <Self as Ivc<T::E1, T::E2, T::FC1, T::FC2>>::trusted_setup(rng)?;
            Ok(().into())
        }

        #[pallet::weight(10_000)]
        pub fn verify(
            origin: OriginFor<T>,
            proof: RecursiveProof<T::E1, T::E2, T::FC1, T::FC2>,
        ) -> DispatchResultWithPostInfo {
            ensure_signed(origin)?;
            <Self as Ivc<T::E1, T::E2, T::FC1, T::FC2>>::verify(proof)?;
            Ok(().into())
        }
    }
}

impl<T: Config> Pallet<T> {
    /// The RPC method to get public parameters
    pub fn get_public_params() -> Option<PublicParams<T::E1, T::E2, T::FC1, T::FC2>> {
        PublicParameters::<T>::get()
    }
}

impl<T: Config> Ivc<T::E1, T::E2, T::FC1, T::FC2> for Pallet<T> {
    /// The API method to setup public parameters
    fn trusted_setup(mut rng: FullcodecRng) -> DispatchResultWithPostInfo {
        match Self::public_params() {
            Some(_) => Err(DispatchErrorWithPostInfo {
                post_info: PostDispatchInfo::from(()),
                error: DispatchError::Other("already setup"),
            }),
            None => {
                let pp = PublicParams::<T::E1, T::E2, T::FC1, T::FC2>::setup(&mut rng);
                PublicParameters::<T>::put(&pp);
                Self::deposit_event(Event::<T>::TrustedSetup(pp));
                Ok(().into())
            }
        }
    }

    /// The API method to verify the proof validity
    fn verify(proof: RecursiveProof<T::E1, T::E2, T::FC1, T::FC2>) -> DispatchResultWithPostInfo {
        match Self::public_params() {
            Some(pp) => match proof.verify(&pp) {
                true => Ok(().into()),
                false => Err(DispatchErrorWithPostInfo {
                    post_info: PostDispatchInfo::from(()),
                    error: DispatchError::Other("invalid proof"),
                }),
            },
            None => Err(DispatchErrorWithPostInfo {
                post_info: PostDispatchInfo::from(()),
                error: DispatchError::Other("setup not ready yet"),
            }),
        }
    }
}
