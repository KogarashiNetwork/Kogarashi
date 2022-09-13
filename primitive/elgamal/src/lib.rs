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

#[derive(Debug, Clone, Decode, Encode)]
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
        decrypted_message.add(random * private_key * g.clone());
        decrypted_message.add(-private_key * (random * g));
        decrypted_message.to_affine()
    }

    pub fn add(&self, other: &Self) {}

    pub fn sub(&self, other: &Self) {}
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
        fn test_elgamal(private_key in arb_fr(), m in arb_fr(), r in arb_fr()) {
            let encrypted_balance = EncryptedNumber::encrypt(private_key, m, r);
            let decrypted_message = encrypted_balance.decrypt(private_key, r);
        }
    }
}
