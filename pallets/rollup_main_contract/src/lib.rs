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

use codec::{Decode, Encode};
#[cfg(feature = "std")]
use frame_support::traits::GenesisBuild;
use frame_support::traits::StoredMap;
pub use pallet::*;
use sp_runtime::{
    traits::{CheckedAdd, CheckedSub, MaybeSerializeDeserialize, StaticLookup, StoredMapError},
    DispatchResult, RuntimeDebug,
};
use sp_std::fmt::Debug;
use sp_std::prelude::*;

#[frame_support::pallet]
pub mod pallet {

    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use she_elgamal::ConfidentialTransferPublicInputs;
    use zkstd::common::Pairing;

    #[pallet::config]
    pub trait Config<I: 'static = ()>: frame_system::Config {
        type P: Pairing;
        /// The balance of an account.
        type EncryptedBalance: Parameter
            + CheckedAdd
            + CheckedSub
            + Member
            + Default
            + Copy
            + MaybeSerializeDeserialize
            + Debug
            + ConfidentialTransferPublicInputs<Self::P>;

        /// The overarching event type.
        type Event: From<Event<Self, I>> + IsType<<Self as frame_system::Config>::Event>;

        /// The means of storing the balances of an account.
        type AccountStore: StoredMap<Self::AccountId, AccountData<Self::EncryptedBalance>>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T, I = ()>(PhantomData<(T, I)>);

    #[pallet::hooks]
    impl<T: Config<I>, I: 'static> Hooks<BlockNumberFor<T>> for Pallet<T, I> {}

    #[pallet::call]
    impl<T: Config<I>, I: 'static> Pallet<T, I>
    // where
    //     pallet::Call<T, I>: Codec,
    {

        pub(super) fn deposit(
            origin: OriginFor<T>,
            who: <T::Lookup as StaticLookup>::Source,
            new_balance: T::EncryptedBalance,
        ) -> DispatchResultWithPostInfo {

            ensure_root(origin)?;
            let who = T::Lookup::lookup(who)?;
            Self::mutate_account(&who, |account| {
                account.balance = new_balance;
            })?;

            Self::deposit_event(Event::BalanceSet(who, new_balance));
            Ok(().into())
            self.deposits.push(t);
        }

        pub fn withdraw(
            &self,
            // l2_burn_merkle_proof: MerkleProof<F, H, N>,
            batch_root: F,
            transaction: Transaction,
            l1_address: PublicKey,
        ) {
            // merkle_verify(l2_burn_merkle_proof, batch_root);
            // send(transaction.amount, l1_address);
        }

        pub fn update_state(&mut self, new_state_root: F) {
            self.rollup_state_root = new_state_root;
        }
        pub fn add_batch(
            &mut self,
            proof: Proof<F, H, N, BATCH_SIZE>,
            compressed_batch_data: Batch<F, H, N, BATCH_SIZE>,
        ) {
            assert!(self.verifier_contract.verify_proof(proof));
            self.update_state(compressed_batch_data.final_root());
            self.calldata.push(compressed_batch_data);
        }

        pub fn check_balance(&self, merkle_proof: MerkleProof<F, H, N>) -> u64 {
            // merkle_verify(merkle_proof, self.rollup_state_root);
            // get_balance()
            0
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    #[pallet::metadata(T::AccountId = "AccountId", T::Balance = "Balance")]
    pub enum Event<T: Config<I>, I: 'static = ()> {
        /// An account was created with some free balance. \[account, free_balance\]
        Endowed(T::AccountId, T::EncryptedBalance),
        /// Transfer succeeded. \[from, to, value\]
        Transfer(T::AccountId, T::AccountId, T::EncryptedBalance),
        /// A balance was set by root. \[who, free, reserved\]
        BalanceSet(T::AccountId, T::EncryptedBalance),
    }

    /// The balance of an account.
    ///
    /// NOTE: This is only used in the case that this pallet is used to store balances.
    #[pallet::storage]
    pub type Account<T: Config<I>, I: 'static = ()> =
        StorageMap<_, Blake2_128Concat, T::AccountId, AccountData<T::EncryptedBalance>, ValueQuery>;

    /// Storage version of the pallet.
    ///
    /// This is set to v2.0.0 for new networks.
    #[pallet::storage]
    pub(super) type StorageVersion<T: Config<I>, I: 'static = ()> =
        StorageValue<_, Releases, ValueQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config<I>, I: 'static = ()> {
        pub balances: Vec<(T::AccountId, T::EncryptedBalance)>,
    }

    #[cfg(feature = "std")]
    impl<T: Config<I>, I: 'static> Default for GenesisConfig<T, I> {
        fn default() -> Self {
            Self {
                balances: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config<I>, I: 'static> GenesisBuild<T, I> for GenesisConfig<T, I> {
        fn build(&self) {
            <StorageVersion<T, I>>::put(Releases::V2_0_0);

            // ensure no duplicates exist.
            let endowed_accounts = self
                .balances
                .iter()
                .map(|(x, _)| x)
                .cloned()
                .collect::<std::collections::BTreeSet<_>>();

            assert!(
                endowed_accounts.len() == self.balances.len(),
                "duplicate balances in genesis."
            );

            for &(ref who, balance) in self.balances.iter() {
                assert!(T::AccountStore::insert(who, AccountData { balance }).is_ok());
            }
        }
    }
}

#[cfg(feature = "std")]
impl<T: Config<I>, I: 'static> GenesisConfig<T, I> {
    /// Direct implementation of `GenesisBuild::build_storage`.
    ///
    /// Kept in order not to break dependency.
    pub fn build_storage(&self) -> Result<sp_runtime::Storage, String> {
        <Self as GenesisBuild<T, I>>::build_storage(self)
    }

    /// Direct implementation of `GenesisBuild::assimilate_storage`.
    ///
    /// Kept in order not to break dependency.
    pub fn assimilate_storage(&self, storage: &mut sp_runtime::Storage) -> Result<(), String> {
        <Self as GenesisBuild<T, I>>::assimilate_storage(self, storage)
    }
}

/// All balance information for an account.
#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, RuntimeDebug)]
pub struct AccountData<EncryptedBalance> {
    /// Non-reserved part of the balance. There may still be restrictions on this, but it is the
    /// total pool what may in principle be transferred, reserved and used for tipping.
    ///
    /// This is the only balance that matters in terms of most operations on tokens. It
    /// alone is used to determine the balance when in the contract execution environment.
    pub balance: EncryptedBalance,
}

impl<EncryptedBalance: Copy> AccountData<EncryptedBalance> {
    /// The total balance in this account including any that is reserved and ignoring any frozen.
    fn total(&self) -> EncryptedBalance {
        self.balance
    }
}

// A value placed in storage that represents the current version of the Balances storage.
// This value is used by the `on_runtime_upgrade` logic to determine whether we run
// storage migration logic. This should match directly with the semantic versions of the Rust crate.
#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq, RuntimeDebug)]
enum Releases {
    V1_0_0,
    V2_0_0,
}

impl Default for Releases {
    fn default() -> Self {
        Releases::V1_0_0
    }
}

impl<T: Config<I>, I: 'static> Pallet<T, I> {
    fn account(who: &T::AccountId) -> AccountData<T::EncryptedBalance> {
        T::AccountStore::get(who)
    }

    /// Mutate an account to some new value, or delete it entirely with `None`. Will enforce
    /// `ExistentialDeposit` law, annulling the account as needed.
    ///
    /// NOTE: Doesn't do any preparatory work for creating a new account, so should only be used
    /// when it is known that the account already exists.
    ///
    /// NOTE: LOW-LEVEL: This will not attempt to maintain total issuance. It is expected that
    /// the caller will do this.
    pub fn mutate_account<R>(
        who: &T::AccountId,
        f: impl FnOnce(&mut AccountData<T::EncryptedBalance>) -> R,
    ) -> Result<R, StoredMapError> {
        Self::try_mutate_account(who, |a, _| -> Result<R, StoredMapError> { Ok(f(a)) })
    }

    /// Mutate an account to some new value, or delete it entirely with `None`. Will enforce
    /// `ExistentialDeposit` law, annulling the account as needed. This will do nothing if the
    /// result of `f` is an `Err`.
    ///
    /// NOTE: Doesn't do any preparatory work for creating a new account, so should only be used
    /// when it is known that the account already exists.
    ///
    /// NOTE: LOW-LEVEL: This will not attempt to maintain total issuance. It is expected that
    /// the caller will do this.
    fn try_mutate_account<R, E: From<StoredMapError>>(
        who: &T::AccountId,
        f: impl FnOnce(&mut AccountData<T::EncryptedBalance>, bool) -> Result<R, E>,
    ) -> Result<R, E> {
        T::AccountStore::try_mutate_exists(who, |maybe_account| {
            // SBP-M1 review: you can use `match` for handling `maybe_account`
            let is_new = maybe_account.is_none();
            let mut account = maybe_account.take().unwrap_or_default();
            f(&mut account, is_new).map(move |result| {
                // SBP-M1 review: just a note, I prefer `match` instead of `if` `else`
                let maybe_endowed = if is_new { Some(account.balance) } else { None };
                *maybe_account = Some(account);
                (maybe_endowed, result)
            })
        })
        .map(|(maybe_endowed, result)| {
            if let Some(endowed) = maybe_endowed {
                Self::deposit_event(Event::Endowed(who.clone(), endowed));
            }
            result
        })
    }
}
