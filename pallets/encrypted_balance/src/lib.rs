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

//! # Balances Pallet
//!
//! The Balances pallet provides functionality for handling accounts and balances.
//!
//! - [`Config`]
//! - [`Call`]
//! - [`Pallet`]
//!
//! ## Overview
//!
//! The Balances pallet provides functions for:
//!
//! - Getting and setting free balances.
//! - Retrieving total, reserved and unreserved balances.
//! - Repatriating a reserved balance to a beneficiary account that exists.
//! - Transferring a balance between accounts (when not reserved).
//! - Slashing an account balance.
//! - Account creation and removal.
//! - Managing total issuance.
//! - Setting and managing locks.
//!
//! ### Terminology
//!
//! - **Existential Deposit:** The minimum balance required to create or keep an account open. This prevents
//! "dust accounts" from filling storage. When the free plus the reserved balance (i.e. the total balance)
//!   fall below this, then the account is said to be dead; and it loses its functionality as well as any
//!   prior history and all information on it is removed from the chain's state.
//!   No account should ever have a total balance that is strictly between 0 and the existential
//!   deposit (exclusive). If this ever happens, it indicates either a bug in this pallet or an
//!   erroneous raw mutation of storage.
//!
//! - **Total Issuance:** The total number of units in existence in a system.
//!
//! - **Reaping an account:** The act of removing an account by resetting its nonce. Happens after its
//! total balance has become zero (or, strictly speaking, less than the Existential Deposit).
//!
//! - **Free Balance:** The portion of a balance that is not reserved. The free balance is the only
//!   balance that matters for most operations.
//!
//! - **Reserved Balance:** Reserved balance still belongs to the account holder, but is suspended.
//!   Reserved balance can still be slashed, but only after all the free balance has been slashed.
//!
//! - **Imbalance:** A condition when some funds were credited or debited without equal and opposite accounting
//! (i.e. a difference between total issuance and account balances). Functions that result in an imbalance will
//! return an object of the `Imbalance` trait that can be managed within your runtime logic. (If an imbalance is
//! simply dropped, it should automatically maintain any book-keeping such as total issuance.)
//!
//! - **Lock:** A freeze on a specified amount of an account's free balance until a specified block number. Multiple
//! locks always operate over the same funds, so they "overlay" rather than "stack".
//!
//! ### Implementations
//!
//! The Balances pallet provides implementations for the following traits. If these traits provide the functionality
//! that you need, then you can avoid coupling with the Balances pallet.
//!
//! - [`Currency`](frame_support::traits::Currency): Functions for dealing with a
//! fungible assets system.
//! - [`ReservableCurrency`](frame_support::traits::ReservableCurrency):
//! Functions for dealing with assets that can be reserved from an account.
//! - [`LockableCurrency`](frame_support::traits::LockableCurrency): Functions for
//! dealing with accounts that allow liquidity restrictions.
//! - [`Imbalance`](frame_support::traits::Imbalance): Functions for handling
//! imbalances between total issuance in the system and account balances. Must be used when a function
//! creates new funds (e.g. a reward) or destroys some funds (e.g. a system fee).
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! - `transfer` - Transfer some liquid free balance to another account.
//! - `set_balance` - Set the balances of a given account. The origin of this call must be root.
//!
//! ## Usage
//!
//! The following examples show how to use the Balances pallet in your custom pallet.
//!
//! ### Examples from the FRAME
//!
//! The Contract pallet uses the `Currency` trait to handle gas payment, and its types inherit from `Currency`:
//!
//! ```
//! use frame_support::traits::Currency;
//! # pub trait Config: frame_system::Config {
//! #   type Currency: Currency<Self::AccountId>;
//! # }
//!
//! pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
//! pub type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::NegativeImbalance;
//!
//! # fn main() {}
//! ```
//!
//! The Staking pallet uses the `LockableCurrency` trait to lock a stash account's funds:
//!
//! ```
//! use frame_support::traits::{WithdrawReasons, LockableCurrency};
//! use sp_runtime::traits::Bounded;
//! pub trait Config: frame_system::Config {
//!     type Currency: LockableCurrency<Self::AccountId, Moment=Self::BlockNumber>;
//! }
//! # struct StakingLedger<T: Config> {
//! #   stash: <T as frame_system::Config>::AccountId,
//! #   total: <<T as Config>::Currency as frame_support::traits::Currency<<T as frame_system::Config>::AccountId>>::Balance,
//! #   phantom: std::marker::PhantomData<T>,
//! # }
//! # const STAKING_ID: [u8; 8] = *b"staking ";
//!
//! fn update_ledger<T: Config>(
//!     controller: &T::AccountId,
//!     ledger: &StakingLedger<T>
//! ) {
//!     T::Currency::set_lock(
//!         STAKING_ID,
//!         &ledger.stash,
//!         ledger.total,
//!         WithdrawReasons::all()
//!     );
//!     // <Ledger<T>>::insert(controller, ledger); // Commented out as we don't have access to Staking's storage here.
//! }
//! # fn main() {}
//! ```
//!
//! ## Genesis config
//!
//! The Balances pallet depends on the [`GenesisConfig`].
//!
//! ## Assumptions
//!
//! * Total issued balanced of all accounts should be less than `Config::Balance::max_value()`.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

#[macro_use]
mod tests;
mod encrypted_currency;
#[cfg(feature = "std")]
mod tests_composite;
mod tests_local;
pub mod weights;

use codec::{Decode, Encode};
pub use encrypted_currency::EncryptedCurrency;
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
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {

    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use she_elgamal::ConfidentialTransferPublicInputs;
    use zkstd::common::TwistedEdwardsAffine;

    #[pallet::config]
    pub trait Config<I: 'static = ()>: frame_system::Config {
        type Affine: TwistedEdwardsAffine;

        /// The balance of an account.
        type EncryptedBalance: Parameter
            + Eq
            + PartialEq
            + CheckedAdd
            + CheckedSub
            + Member
            + Default
            + Copy
            + MaybeSerializeDeserialize
            + Debug
            + ConfidentialTransferPublicInputs<Self::Affine>;

        /// The overarching event type.
        type Event: From<Event<Self, I>> + IsType<<Self as frame_system::Config>::Event>;

        /// The means of storing the balances of an account.
        type AccountStore: StoredMap<Self::AccountId, AccountData<Self::EncryptedBalance>>;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
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
        /// Transfer some liquid free balance to another account.
        ///
        /// `transfer` will set the `FreeBalance` of the sender and receiver.
        /// It will decrease the total issuance of the system by the `TransferFee`.
        /// If the sender's account is below the existential deposit as a result
        /// of the transfer, the account will be reaped.
        ///
        /// The dispatch origin for this call must be `Signed` by the transactor.
        ///
        /// # <weight>
        /// - Dependent on arguments but not critical, given proper implementations for
        ///   input config types. See related functions below.
        /// - It contains a limited number of reads and writes internally and no complex computation.
        ///
        /// Related functions:
        ///
        ///   - `ensure_can_withdraw` is always called internally but has a bounded complexity.
        ///   - Transferring balances to accounts that did not exist before will cause
        ///      `T::OnNewAccount::on_new_account` to be called.
        ///   - Removing enough funds from an account will trigger `T::DustRemoval::on_unbalanced`.
        ///   - `transfer_keep_alive` works the same way as `transfer`, but has an additional
        ///     check that the transfer will not kill the origin account.
        /// ---------------------------------
        /// - Base Weight: 73.64 µs, worst case scenario (account created, account removed)
        /// - DB Weight: 1 Read and 1 Write to destination account
        /// - Origin account is already in memory, so no DB operations for them.
        /// # </weight>
        #[pallet::weight(T::WeightInfo::transfer())]
        pub fn transfer(
            origin: OriginFor<T>,
            dest: <T::Lookup as StaticLookup>::Source,
            sender_amount: T::EncryptedBalance,
            recipient_amount: T::EncryptedBalance,
        ) -> DispatchResultWithPostInfo {
            let transactor = ensure_signed(origin)?;
            let dest = T::Lookup::lookup(dest)?;
            <Self as EncryptedCurrency<_, _>>::transfer(
                &transactor,
                &dest,
                sender_amount,
                recipient_amount,
            )?;
            Ok(().into())
        }

        /// Set the balances of a given account.
        ///
        /// This will alter `FreeBalance` and `ReservedBalance` in storage. it will
        /// also decrease the total issuance of the system (`TotalIssuance`).
        /// If the new free or reserved balance is below the existential deposit,
        /// it will reset the account nonce (`frame_system::AccountNonce`).
        ///
        /// The dispatch origin for this call is `root`.
        ///
        /// # <weight>
        /// - Independent of the arguments.
        /// - Contains a limited number of reads and writes.
        /// ---------------------
        /// - Base Weight:
        ///     - Creating: 27.56 µs
        ///     - Killing: 35.11 µs
        /// - DB Weight: 1 Read, 1 Write to `who`
        /// # </weight>
        #[pallet::weight(
        	T::WeightInfo::set_balance_creating() // Creates a new account.
        		.max(T::WeightInfo::set_balance_killing()) // Kills an existing account.
        )]
        pub(super) fn set_balance(
            origin: OriginFor<T>,
            who: <T::Lookup as StaticLookup>::Source,
            new_balance: T::EncryptedBalance,
        ) -> DispatchResultWithPostInfo {
            // SBP-M1 review: Think about using some onchain decentralized entity
            // instead of Sudo Pallet.
            ensure_root(origin)?;
            let who = T::Lookup::lookup(who)?;
            Self::mutate_account(&who, |account| {
                account.balance = new_balance;
            })?;

            Self::deposit_event(Event::BalanceSet(who, new_balance));
            Ok(().into())
        }

        /// Exactly as `transfer`, except the origin must be root and the source account may be
        /// specified.
        /// # <weight>
        /// - Same as transfer, but additional read and write because the source account is
        ///   not assumed to be in the overlay.
        /// # </weight>
        #[pallet::weight(T::WeightInfo::force_transfer())]
        pub fn force_transfer(
            origin: OriginFor<T>,
            source: <T::Lookup as StaticLookup>::Source,
            dest: <T::Lookup as StaticLookup>::Source,
            sender_amount: T::EncryptedBalance,
            recipient_amount: T::EncryptedBalance,
        ) -> DispatchResultWithPostInfo {
            // SBP-M1 review: Think about using some onchain decentralized entity
            // instead of Sudo Pallet.
            ensure_root(origin)?;
            let source = T::Lookup::lookup(source)?;
            let dest = T::Lookup::lookup(dest)?;
            <Self as EncryptedCurrency<_, _>>::transfer(
                &source,
                &dest,
                sender_amount,
                recipient_amount,
            )?;
            Ok(().into())
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

// Custom impl
impl<T: Config<I>, I: 'static> EncryptedCurrency<T::AccountId, T::EncryptedBalance> for Pallet<T, I>
where
    T::EncryptedBalance: MaybeSerializeDeserialize + Debug,
{
    fn total_balance(who: &T::AccountId) -> T::EncryptedBalance {
        Self::account(who).total()
    }

    // Transfer some free balance from `transactor` to `dest`, respecting existence requirements.
    // Is a no-op if value to be transferred is zero or the `transactor` is the same as `dest`.
    fn transfer(
        transactor: &T::AccountId,
        dest: &T::AccountId,
        sender_amount: T::EncryptedBalance,
        recipient_amount: T::EncryptedBalance,
    ) -> DispatchResult {
        if transactor == dest {
            return Ok(());
        }

        Self::try_mutate_account(dest, |to_account, _| -> DispatchResult {
            Self::try_mutate_account(transactor, |from_account, _| -> DispatchResult {
                from_account.balance = from_account.balance - sender_amount;
                to_account.balance = to_account.balance + recipient_amount;

                Ok(())
            })
        })?;

        // Emit transfer event.
        Self::deposit_event(Event::Transfer(
            // SBP-M1 review: try to avoid cloning values
            transactor.clone(),
            dest.clone(),
            sender_amount,
        ));

        Ok(())
    }

    fn deposit_creating(who: &T::AccountId, value: T::EncryptedBalance) -> DispatchResult {
        Self::try_mutate_account(who, |account, _| -> DispatchResult {
            account.balance = value;
            Ok(())
        })
    }
}
