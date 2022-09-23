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
    fr::Fr,
};

#[derive(Debug, Clone, Decode, Encode, PartialEq, Eq)]
pub struct EncryptedNumber {
    s: Affine,
    t: Affine,
}

#[allow(unused_variables)]
impl EncryptedNumber {
    pub fn encrypt(private_key: Fr, value: Fr, random: Fr) -> Self {
        let g = Projective::generator();
        let public_key = private_key * g.clone();
        let mut left = value * g.clone();
        left.add(random * public_key);
        EncryptedNumber {
            s: left.to_affine(),
            t: (random * g.clone()).to_affine(),
        }
    }

    pub fn decrypt(&self, private_key: Fr, random: Fr) -> Affine {
        let g = Projective::generator();
        let mut decrypted_message = Projective::from(self.s.clone());
        // decrypted_message.add(random * private_key * g.clone());
        // let neg = (private_key * (random * g)).neg();
        let neg = (private_key * Projective::from(self.t.clone())).neg();
        decrypted_message.add(neg);
        decrypted_message.to_affine()
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
    use zero_jubjub::fr::Fr;

    use crate::EncryptedNumber;

    prop_compose! {
        fn arb_fr()(bytes in [any::<u8>(); 16]) -> Fr {
            Fr::random(XorShiftRng::from_seed(bytes))
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1))]
        #[test]
        fn test_elgamal(alice_pk in arb_fr(), bob_pk in arb_fr()) {
            let alice_balance = Fr::from_hex("0xA").unwrap();
            let bob_balance = Fr::from_hex("0x0").unwrap();
            let transfer_amount = Fr::from_hex("0x5").unwrap();
            let alice_randomness = Fr::from_hex("0x315").unwrap();
            let bob_randomness = Fr::from_hex("0x1c8").unwrap();
            let alice_transfer_randomness = Fr::from_hex("0x7b").unwrap();
            libc_print::libc_println!("Alice: {alice_balance}, Bob: {bob_balance}");
            let alice_balance_enc = EncryptedNumber::encrypt(alice_pk, alice_balance, alice_randomness);
            let bob_balance_enc = EncryptedNumber::encrypt(bob_pk, bob_balance, bob_randomness);

            let transfer_amount_enc_alice = EncryptedNumber::encrypt(alice_pk, transfer_amount, alice_transfer_randomness);
            let transfer_amount_enc_bob = EncryptedNumber::encrypt(bob_pk, transfer_amount, alice_transfer_randomness);
            let alice_after_balance_enc = alice_balance_enc.sub(&transfer_amount_enc_alice);
            let bob_after_balance_enc = bob_balance_enc.add(&transfer_amount_enc_bob);

            let alice_randomness_sum = alice_randomness - alice_transfer_randomness;
            let bob_randomness_sum = bob_randomness + alice_transfer_randomness;
            let explicit_alice = alice_balance - transfer_amount;
            let explicit_bob = bob_balance + transfer_amount;
            libc_print::libc_println!("Alice: {explicit_alice}, Bob: {explicit_bob}");
            let exp_alice_balance_enc = EncryptedNumber::encrypt(alice_pk, explicit_alice, alice_randomness_sum);
            let exp_bob_balance_enc = EncryptedNumber::encrypt(bob_pk, explicit_bob, bob_randomness_sum);
            libc_print::libc_println!("Enc_Alice: {alice_after_balance_enc:?}\nEnc_Bob: {bob_after_balance_enc:?}\n");
            libc_print::libc_println!("Enc_Alice: {exp_alice_balance_enc:?}\nEnc_Bob: {exp_bob_balance_enc:?}");
            // assert_eq!(exp_alice_balance_enc, alice_after_balance_enc);
            // assert_eq!(exp_bob_balance_enc, bob_after_balance_enc);
        }
    }
}
