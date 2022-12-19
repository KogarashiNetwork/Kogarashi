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
    ($test:ty, $ext_builder:ty) => {
        use frame_support::{assert_noop, assert_ok};
        use frame_system::RawOrigin;
        use sp_runtime::traits::BadOrigin;
        use $crate::*;
        use zero_elgamal::Fr;

        const ID_1_PK: Fr = Fr::to_mont_form([1, 0, 0, 0]);
        const ID_2_PK: Fr = Fr::to_mont_form([2, 0, 0, 0]);
        const ID_1_RANDOM: Fr = Fr::to_mont_form([1, 2, 3, 4]);
        const ID_2_RANDOM: Fr = Fr::to_mont_form([4, 3, 2, 1]);
        const TRANSFER_RANDOM: Fr = Fr::to_mont_form([4, 2, 1, 3]);

        #[test]
        fn balance_transfer_works() {
            let balance1 = 50;
            let transfer = 20;
            let enc_balance1 = EncryptedNumber::encrypt(ID_1_PK, balance1, ID_1_RANDOM);
            let enc_balance2 = EncryptedNumber::encrypt(ID_2_PK, 0, ID_2_RANDOM);
            let enc_transfer = EncryptedNumber::encrypt(ID_1_PK, transfer, TRANSFER_RANDOM);

            <$ext_builder>::default().build().execute_with(|| {
                let _ = EncryptedBalances::deposit_creating(&1, enc_balance1);
                assert_ok!(EncryptedBalances::transfer(Some(1).into(), 2, enc_transfer));
                assert_eq!(
                    EncryptedBalances::total_balance(&1),
                    enc_balance1 - enc_transfer
                );
                assert_eq!(
                    EncryptedBalances::total_balance(&2),
                    enc_balance2 + enc_transfer
                );
            });
        }

        #[test]
        fn force_transfer_works() {
            let balance1 = 50;
            let transfer = 20;
            let enc_balance1 = EncryptedNumber::encrypt(ID_1_PK, balance1, ID_1_RANDOM);
            let enc_balance2 = EncryptedNumber::encrypt(ID_2_PK, 0, ID_2_RANDOM);
            let enc_transfer = EncryptedNumber::encrypt(ID_1_PK, transfer, TRANSFER_RANDOM);
            <$ext_builder>::default().build().execute_with(|| {
                let _ = EncryptedBalances::deposit_creating(&1, enc_balance1);
                assert_noop!(
                    EncryptedBalances::force_transfer(Some(2).into(), 1, 2, enc_transfer),
                    BadOrigin,
                );
                assert_ok!(EncryptedBalances::force_transfer(
                    RawOrigin::Root.into(),
                    1,
                    2,
                    enc_transfer
                ));
                assert_eq!(
                    EncryptedBalances::total_balance(&1),
                    enc_balance1 - enc_transfer
                );
                assert_eq!(
                    EncryptedBalances::total_balance(&2),
                    enc_balance2 + enc_transfer
                );
            });
        }
    };
}
