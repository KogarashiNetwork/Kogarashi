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

pub use pallet::*;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod mock;

mod circuit;
mod traits;

pub use circuit::ConfidentialTransferTransaction;
pub use traits::ConfidentialTransfer;

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use pallet_encrypted_balance::EncryptedCurrency;
use pallet_plonk::Plonk;
use pallet_plonk::{FullcodecRng, Proof};

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use sp_runtime::traits::StaticLookup;

    #[pallet::config]
    pub trait Config:
        frame_system::Config + pallet_plonk::Config + pallet_encrypted_balance::Config
    {
        type Plonk: Plonk<Self::AccountId, <Self as pallet_encrypted_balance::Config>::P>;
        type EncryptedCurrency: EncryptedCurrency<Self::AccountId, Self::EncryptedBalance>;
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::event]
    pub enum Event<T: Config> {}

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
            rng: FullcodecRng,
        ) -> DispatchResultWithPostInfo {
            pallet_plonk::Pallet::<T>::trusted_setup(origin, degree, rng)?;
            Ok(().into())
        }

        #[pallet::weight(10_000)]
        pub fn confidential_transfer(
            origin: OriginFor<T>,
            dest: <T::Lookup as StaticLookup>::Source,
            proof: Proof<<T as pallet_encrypted_balance::Config>::P>,
            transaction_params: ConfidentialTransferTransaction<
                <T as pallet_encrypted_balance::Config>::EncryptedBalance,
                <T as pallet_encrypted_balance::Config>::P,
            >,
        ) -> DispatchResultWithPostInfo {
            let transactor = ensure_signed(origin)?;
            let dest = T::Lookup::lookup(dest)?;
            <Self as ConfidentialTransfer<_, <T as pallet_encrypted_balance::Config>::P>>::confidential_transfer(
                &transactor,
                &dest,
                proof,
                transaction_params,
            )?;
            Ok(().into())
        }
    }
}

impl<T: Config> ConfidentialTransfer<T::AccountId, <T as pallet_encrypted_balance::Config>::P>
    for Pallet<T>
{
    type EncryptedBalance = T::EncryptedBalance;

    fn total_balance(who: &T::AccountId) -> Self::EncryptedBalance {
        T::EncryptedCurrency::total_balance(who)
    }

    fn trusted_setup(
        who: &T::AccountId,
        val: u32,
        rng: FullcodecRng,
    ) -> DispatchResultWithPostInfo {
        T::Plonk::trusted_setup(who, val, rng)
    }

    fn confidential_transfer(
        who: &T::AccountId,
        dest: &T::AccountId,
        proof: pallet_plonk::Proof<<T as pallet_encrypted_balance::Config>::P>,
        transaction_params: ConfidentialTransferTransaction<
            Self::EncryptedBalance,
            <T as pallet_encrypted_balance::Config>::P,
        >,
    ) -> DispatchResultWithPostInfo {
        let public_inputs = transaction_params.clone().public_inputs();
        let (sender_amount, recipient_amount) = transaction_params.transaction_amount();
        T::Plonk::verify(who, proof, public_inputs.to_vec())?;
        T::EncryptedCurrency::transfer(who, dest, sender_amount, recipient_amount)?;
        Ok(().into())
    }
}
