// This file is part of Substrate.

// Copyright (C) 2017-2021 Parity Technologies (UK) Ltd.
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

//! Macro for creating the tests for the module.

#![cfg(test)]

#[macro_export]
macro_rules! decl_tests {
    ($test:ty, $ext_builder:ty, $existential_deposit:expr) => {
        use crate::*;
        use frame_support::{
            assert_err, assert_noop, assert_ok, assert_storage_noop,
            traits::{
                Currency, ExistenceRequirement::AllowDeath, LockIdentifier, LockableCurrency,
                ReservableCurrency, WithdrawReasons,
            },
            StorageValue,
        };
        use frame_system::RawOrigin;
        use pallet_transaction_payment::{ChargeTransactionPayment, Multiplier};
        use sp_runtime::{
            traits::{BadOrigin, SignedExtension},
            FixedPointNumber,
        };

        const ID_1: LockIdentifier = *b"1       ";
        const ID_2: LockIdentifier = *b"2       ";

        pub const CALL: &<$test as frame_system::Config>::Call =
            &Call::Balances(pallet_balances::Call::transfer(0, 0));

        /// create a transaction info struct from weight. Handy to avoid building the whole struct.
        pub fn info_from_weight(w: Weight) -> DispatchInfo {
            DispatchInfo {
                weight: w,
                ..Default::default()
            }
        }

        fn events() -> Vec<Event> {
            let evt = System::events()
                .into_iter()
                .map(|evt| evt.event)
                .collect::<Vec<_>>();

            System::reset_events();

            evt
        }

        fn last_event() -> Event {
            system::Module::<Test>::events()
                .pop()
                .expect("Event expected")
                .event
        }

        #[test]
        fn balance_transfer_works() {
            <$ext_builder>::default().build().execute_with(|| {
                let _ = Balances::deposit_creating(&1, 111);
                assert_ok!(Balances::transfer(Some(1).into(), 2, 69));
                assert_eq!(Balances::total_balance(&1), 42);
                assert_eq!(Balances::total_balance(&2), 69);
            });
        }

        #[test]
        fn force_transfer_works() {
            <$ext_builder>::default().build().execute_with(|| {
                let _ = Balances::deposit_creating(&1, 111);
                assert_noop!(
                    Balances::force_transfer(Some(2).into(), 1, 2, 69),
                    BadOrigin,
                );
                assert_ok!(Balances::force_transfer(RawOrigin::Root.into(), 1, 2, 69));
                assert_eq!(Balances::total_balance(&1), 42);
                assert_eq!(Balances::total_balance(&2), 69);
            });
        }
    };
}
