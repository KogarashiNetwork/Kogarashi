// Copyright (C) 2021-2022 Artree (JP) LLC.
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

//! # Lifted-ElGamal Pallet
//!
//! ## Overview
//!
//! This is the additive homomorphic encryption which supports one-time multiplication.
//! This library is implemented based on [original paper](https://github.com/herumi/mcl/blob/master/misc/she/she.pdf).

#![cfg_attr(not(feature = "std"), no_std)]

use parity_scale_codec::{Decode, Encode};
use zero_jubjub::{
    coordinate::{Affine, Projective},
    fr::{Fr, FrRaw},
    interface::Coordinate,
};

#[derive(Debug, Clone, Decode, Encode, PartialEq, Eq)]
pub struct EncryptedNumber {
    s: Affine,
    t: Affine,
}

#[allow(unused_variables)]
impl EncryptedNumber {
    pub fn encrypt(private_key: Fr, value: u32, random: Fr) -> Self {
        let g = Projective::generator();
        let public_key = private_key * g.clone();
        let mut left = Fr(FrRaw::from(value)) * g.clone();
        left.add(random * public_key);
        EncryptedNumber {
            s: left.to_affine(),
            t: (random * g).to_affine(),
        }
    }

    pub fn decrypt(&self, private_key: Fr) -> Option<u32> {
        let g = Projective::generator();
        let mut decrypted_message = Projective::from(self.s.clone());
        let neg = (private_key * Projective::from(self.t.clone())).neg();
        decrypted_message.add(neg);

        let mut acc = Projective::identity();
        for i in 0..150000 {
            if acc == decrypted_message {
                return Some(i);
            }
            acc.add(g.clone());
        }
        None
    }

    pub fn add(&self, other: &Self) -> Self {
        let mut s = Projective::from(self.s.clone());
        let mut t = Projective::from(self.t.clone());
        s.add(Projective::from(other.s.clone()));
        t.add(Projective::from(other.t.clone()));

        Self {
            s: s.to_affine(),
            t: t.to_affine(),
        }
    }

    pub fn sub(&self, other: &Self) -> Self {
        let mut s = Projective::from(self.s.clone());
        let mut t = Projective::from(self.t.clone());
        s.add(Projective::from(other.s.clone()).neg());
        t.add(Projective::from(other.t.clone()).neg());

        Self {
            s: s.to_affine(),
            t: t.to_affine(),
        }
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use rand::SeedableRng;
    use rand_xorshift::XorShiftRng;
    use zero_jubjub::fr::{Fr, FrRaw};

    use crate::EncryptedNumber;

    prop_compose! {
        fn arb_fr()(bytes in [any::<u8>(); 16]) -> Fr {
            Fr::random(XorShiftRng::from_seed(bytes))
        }
    }
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(25))]
        #[test]
        fn test_encrypt_decrypt(priv_k in arb_fr(), random in arb_fr(), balance in any::<u16>()) {
            let enc_balance = EncryptedNumber::encrypt(priv_k, balance as u32, random);

            let dec_balance = enc_balance.decrypt(priv_k);
            assert_eq!(dec_balance.unwrap(), balance as u32);
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(25))]
        #[test]
        fn test_homomorphic(
            priv_k in arb_fr(), random1 in arb_fr(), random2 in arb_fr(),
            balance1 in any::<u16>(), balance2 in any::<u16>()
        ) {
            let (balance1, balance2) = if balance1 > balance2 {
                (balance1 as u32, balance2 as u32)
            } else {
                (balance2 as u32, balance1 as u32)
            };

            let enc_balance1 = EncryptedNumber::encrypt(priv_k, balance1, random1);
            let enc_balance2 = EncryptedNumber::encrypt(priv_k, balance2, random2);
            let enc_sub = enc_balance1.sub(&enc_balance2);
            let enc_add = enc_balance1.add(&enc_balance2);

            let dec_sub = enc_sub.decrypt(priv_k);
            let dec_add = enc_add.decrypt(priv_k);

            assert_eq!(dec_sub.unwrap(), balance1 - balance2);
            assert_eq!(dec_add.unwrap(), balance1 + balance2);
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        #[test]
        fn test_elgamal(
            alice_pk in arb_fr(), bob_pk in arb_fr(), alice_balance in 15..u16::MAX, bob_balance in 10..u16::MAX,
            transfer_amount in 10..u16::MAX, alice_randomness in 10..u64::MAX, bob_randomness in 10..u64::MAX,
            alice_transfer_randomness in 10..u64::MAX
        ) {
            let (alice_balance, transfer_amount) = if alice_balance > transfer_amount {
                (alice_balance as u32, transfer_amount as u32)
            } else {
                (transfer_amount as u32, alice_balance as u32)
            };
            let bob_balance = bob_balance as u32;

            // TODO
            let (alice_randomness, alice_transfer_randomness) = if alice_randomness > alice_transfer_randomness {
                (alice_randomness, alice_transfer_randomness)
            } else {
                (alice_transfer_randomness, alice_randomness)
            };
            let alice_randomness = Fr(FrRaw::from(alice_randomness));
            let bob_randomness = Fr(FrRaw::from(bob_randomness));
            let alice_transfer_randomness = Fr(FrRaw::from(alice_transfer_randomness));

            let alice_balance_enc = EncryptedNumber::encrypt(alice_pk, alice_balance, alice_randomness);
            let bob_balance_enc = EncryptedNumber::encrypt(bob_pk, bob_balance, bob_randomness);

            let transfer_amount_enc_alice =
                EncryptedNumber::encrypt(alice_pk, transfer_amount, alice_transfer_randomness);
            let transfer_amount_enc_bob =
                EncryptedNumber::encrypt(bob_pk, transfer_amount, alice_transfer_randomness);

            let alice_after_balance_enc = alice_balance_enc.sub(&transfer_amount_enc_alice);
            let bob_after_balance_enc = bob_balance_enc.add(&transfer_amount_enc_bob);

            let alice_randomness_sum = alice_randomness - alice_transfer_randomness;
            let bob_randomness_sum = bob_randomness + alice_transfer_randomness;

            let explicit_alice = alice_balance - transfer_amount;
            let explicit_bob = bob_balance + transfer_amount;
            let exp_alice_balance_enc =
                EncryptedNumber::encrypt(alice_pk, explicit_alice, alice_randomness_sum);
            let exp_bob_balance_enc =
                EncryptedNumber::encrypt(bob_pk, explicit_bob, bob_randomness_sum);

            assert_eq!(exp_alice_balance_enc, alice_after_balance_enc);
            assert_eq!(exp_bob_balance_enc, bob_after_balance_enc);
        }
    }
}
