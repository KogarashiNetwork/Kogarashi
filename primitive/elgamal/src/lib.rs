// Copyright (C) 2023-2024 Invers (JP) INC.
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

#![no_std]
#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

use core::ops::{Add, Sub};
use ec_pairing::TatePairing;
use jub_jub::{Fp, JubjubAffine, JubjubExtended};
use num_traits::{CheckedAdd, CheckedSub};
use parity_scale_codec::{Decode, Encode};
use serde::{Deserialize, Serialize};
use zkstd::common::{CurveGroup, Pairing};

/// Number encrypted by ElGamal encryption
#[derive(Debug, Clone, Copy, Encode, Decode, PartialEq, Eq, Deserialize, Serialize)]
pub struct EncryptedNumber {
    s: JubjubAffine,
    t: JubjubAffine,
}

impl Default for EncryptedNumber {
    fn default() -> Self {
        Self {
            s: JubjubAffine::ADDITIVE_IDENTITY,
            t: JubjubAffine::ADDITIVE_IDENTITY,
        }
    }
}

// SBP-M1 review: use safe math operations like `checked_add`, `checked_mul`, etc.

impl EncryptedNumber {
    /// Init encrypted number
    pub fn new(s: JubjubAffine, t: JubjubAffine) -> Self {
        Self { s, t }
    }

    /// Enctypt number by private key
    pub fn encrypt(private_key: Fp, value: u32, random: Fp) -> Self {
        let g = JubjubExtended::ADDITIVE_GENERATOR;
        let public_key = g * private_key;
        let left = g * Fp::from(value as u64) + public_key * random;
        EncryptedNumber {
            s: JubjubAffine::from(left),
            t: JubjubAffine::from(g * random),
        }
    }

    /// Decrypt encrypted number by brute force
    pub fn decrypt(&self, private_key: Fp) -> Option<u32> {
        let g = JubjubExtended::ADDITIVE_GENERATOR;
        let decrypted_message =
            JubjubExtended::from(self.s) - (JubjubExtended::from(self.t) * private_key);

        let mut acc = JubjubExtended::ADDITIVE_IDENTITY;
        for i in 0..150000 {
            if acc == decrypted_message {
                return Some(i);
            }
            acc += g;
        }
        None
    }

    /// Get left and right affine point
    pub fn get_coordinate(self) -> (JubjubAffine, JubjubAffine) {
        (self.s, self.t)
    }
}

impl Add for EncryptedNumber {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            s: JubjubAffine::from(JubjubExtended::from(self.s) + JubjubExtended::from(rhs.s)),
            t: JubjubAffine::from(JubjubExtended::from(self.t) + JubjubExtended::from(rhs.t)),
        }
    }
}

impl Sub for EncryptedNumber {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            s: JubjubAffine::from(JubjubExtended::from(self.s) - JubjubExtended::from(rhs.s)),
            t: JubjubAffine::from(JubjubExtended::from(self.t) - JubjubExtended::from(rhs.t)),
        }
    }
}

impl CheckedAdd for EncryptedNumber {
    #[inline]
    fn checked_add(&self, rhs: &Self) -> Option<Self> {
        Some(Self {
            s: JubjubAffine::from(JubjubExtended::from(self.s) + JubjubExtended::from(rhs.s)),
            t: JubjubAffine::from(JubjubExtended::from(self.t) + JubjubExtended::from(rhs.t)),
        })
    }
}

impl CheckedSub for EncryptedNumber {
    #[inline]
    fn checked_sub(&self, rhs: &Self) -> Option<Self> {
        Some(Self {
            s: JubjubAffine::from(JubjubExtended::from(self.s) - JubjubExtended::from(rhs.s)),
            t: JubjubAffine::from(JubjubExtended::from(self.t) - JubjubExtended::from(rhs.t)),
        })
    }
}

/// interface for circuit public inputs
pub trait ConfidentialTransferPublicInputs<P: Pairing> {
    /// init transfer amount public
    fn init(s: P::JubjubAffine, t: P::JubjubAffine) -> Self;

    /// get s and t cypher text
    fn get(self) -> (P::JubjubAffine, P::JubjubAffine);
}

impl ConfidentialTransferPublicInputs<TatePairing> for EncryptedNumber {
    fn init(s: JubjubAffine, t: JubjubAffine) -> Self {
        Self::new(s, t)
    }

    fn get(self) -> (JubjubAffine, JubjubAffine) {
        self.get_coordinate()
    }
}

#[cfg(test)]
mod tests {
    use jub_jub::Fp;
    use rand::{thread_rng, Rng};
    use rand_core::OsRng;
    use zkstd::behave::*;

    use crate::EncryptedNumber;

    fn arb_fr() -> Fp {
        Fp::random(OsRng)
    }

    #[test]
    fn test_encrypt_decrypt() {
        let priv_k = arb_fr();
        let random = arb_fr();
        let balance = thread_rng().gen::<u16>();
        let enc_balance = EncryptedNumber::encrypt(priv_k, balance as u32, random);

        let dec_balance = enc_balance.decrypt(priv_k);
        assert_eq!(dec_balance.unwrap(), balance as u32);
    }

    #[test]
    fn test_homomorphic() {
        let priv_k = arb_fr();
        let random1 = arb_fr();
        let random2 = arb_fr();
        let balance1 = thread_rng().gen::<u16>();
        let balance2 = thread_rng().gen::<u16>();
        let (balance1, balance2) = if balance1 > balance2 {
            (balance1 as u32, balance2 as u32)
        } else {
            (balance2 as u32, balance1 as u32)
        };

        let enc_balance1 = EncryptedNumber::encrypt(priv_k, balance1, random1);
        let enc_balance2 = EncryptedNumber::encrypt(priv_k, balance2, random2);
        let enc_sub = enc_balance1 - enc_balance2;
        let enc_add = enc_balance1 + enc_balance2;

        let dec_sub = enc_sub.decrypt(priv_k);
        let dec_add = enc_add.decrypt(priv_k);

        assert_eq!(dec_sub.unwrap(), balance1 - balance2);
        assert_eq!(dec_add.unwrap(), balance1 + balance2);
    }

    #[test]
    fn test_elgamal() {
        let alice_pk = arb_fr();
        let bob_pk = arb_fr();
        let alice_balance = thread_rng().gen::<u16>();
        let bob_balance = thread_rng().gen::<u16>();
        let transfer_amount = thread_rng().gen::<u16>();
        let alice_randomness = thread_rng().gen::<u64>();
        let bob_randomness = thread_rng().gen::<u64>();
        let alice_transfer_randomness = thread_rng().gen::<u64>();

        let (alice_balance, transfer_amount) = if alice_balance > transfer_amount {
            (alice_balance as u32, transfer_amount as u32)
        } else {
            (transfer_amount as u32, alice_balance as u32)
        };
        let bob_balance = bob_balance as u32;

        // TODO
        let (alice_randomness, alice_transfer_randomness) =
            if alice_randomness > alice_transfer_randomness {
                (alice_randomness, alice_transfer_randomness)
            } else {
                (alice_transfer_randomness, alice_randomness)
            };
        let alice_randomness = Fp::from(alice_randomness);
        let bob_randomness = Fp::from(bob_randomness);
        let alice_transfer_randomness = Fp::from(alice_transfer_randomness);

        let alice_balance_enc = EncryptedNumber::encrypt(alice_pk, alice_balance, alice_randomness);
        let bob_balance_enc = EncryptedNumber::encrypt(bob_pk, bob_balance, bob_randomness);

        let transfer_amount_enc_alice =
            EncryptedNumber::encrypt(alice_pk, transfer_amount, alice_transfer_randomness);
        let transfer_amount_enc_bob =
            EncryptedNumber::encrypt(bob_pk, transfer_amount, alice_transfer_randomness);

        let alice_after_balance_enc = alice_balance_enc - transfer_amount_enc_alice;
        let bob_after_balance_enc = bob_balance_enc + transfer_amount_enc_bob;

        let alice_randomness_sum = alice_randomness - alice_transfer_randomness;
        let bob_randomness_sum = bob_randomness + alice_transfer_randomness;

        let explicit_alice = alice_balance - transfer_amount;
        let explicit_bob = bob_balance + transfer_amount;
        let exp_alice_balance_enc =
            EncryptedNumber::encrypt(alice_pk, explicit_alice, alice_randomness_sum);
        let exp_bob_balance_enc =
            EncryptedNumber::encrypt(bob_pk, explicit_bob, bob_randomness_sum);

        assert_eq!(exp_alice_balance_enc.t, alice_after_balance_enc.t);
        assert_eq!(exp_bob_balance_enc, bob_after_balance_enc);
    }
}
