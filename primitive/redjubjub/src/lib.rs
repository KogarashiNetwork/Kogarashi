// Copyright (C) 2022-2023 Invers (JP) INC.
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

pub mod constant;
mod hash;
mod private_key;
mod public_key;
mod signature;

use bls_12_381::Fr;
pub use hash::sapling_hash;
use jub_jub::{Fp, JubjubAffine, JubjubExtended};
pub use private_key::SecretKey;
pub use public_key::PublicKey;
pub use signature::Signature;
use zkstd::common::RedDSA;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, PartialOrd, Ord)]
pub struct RedJubjub {}

impl RedDSA for RedJubjub {
    type Range = Fr;

    type Scalar = Fp;

    type Affine = JubjubAffine;

    type Extended = JubjubExtended;
}

/// An redjubjub secret key and public key pair.
#[derive(Copy, Clone, Debug)]
pub struct Keypair<P: RedDSA> {
    /// secret key
    pub secret: SecretKey<P>,
    /// public key
    pub public: PublicKey<P>,
}

impl<P: RedDSA> Keypair<P> {
    pub fn new(secret: SecretKey<P>) -> Self {
        let public = secret.to_public_key();
        Self { secret, public }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jub_jub::Fp;
    use rand_core::OsRng;
    use zkstd::common::{Group, SigUtils};

    #[test]
    fn sig_utils() {
        let randomness = OsRng;
        let msg = b"test";
        let secret = SecretKey::<RedJubjub>(Fp::random(OsRng));
        let sig = secret.sign(msg, randomness);
        let pub_key = secret.to_public_key();

        let sig_bytes = sig.to_bytes();
        let sig_back = Signature::from_bytes(sig_bytes).unwrap();
        assert_eq!(sig, sig_back);

        let pub_key_bytes = pub_key.to_bytes();
        let pub_key_back = PublicKey::from_bytes(pub_key_bytes).unwrap();
        assert_eq!(pub_key, pub_key_back);

        let secret_bytes = secret.to_bytes();
        let secret_back = SecretKey::from_bytes(secret_bytes).unwrap();
        assert_eq!(secret, secret_back);
    }

    #[test]
    fn signature_test() {
        for _ in 0..1000 {
            let msg = b"test";
            let wrong_msg = b"tes";
            let randomness = OsRng;

            let priv_key = SecretKey::<RedJubjub>(Fp::random(OsRng));
            let sig = priv_key.sign(msg, randomness);
            let pub_key = priv_key.to_public_key();

            assert!(pub_key.validate(msg, sig));
            assert!(!pub_key.validate(wrong_msg, sig));
        }
    }

    #[test]
    fn rerandomize_test() {
        for _ in 0..1000 {
            let msg = b"test";
            let wrong_msg = b"tes";

            let priv_key = SecretKey::<RedJubjub>(Fp::random(OsRng));
            let pub_key = priv_key.to_public_key();

            // randomization
            let randomize = Fp::random(OsRng);
            let randomize_priv_key = priv_key.randomize_private(randomize);
            let randomize_pub_key = pub_key.randomize_public(randomize);
            let sig = randomize_priv_key.sign(msg, OsRng);

            assert!(randomize_pub_key.validate(msg, sig));
            assert!(!randomize_pub_key.validate(wrong_msg, sig));
        }
    }
}
