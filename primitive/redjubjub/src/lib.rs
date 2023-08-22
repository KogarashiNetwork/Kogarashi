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

mod constant;
mod hash;
mod private_key;
mod public_key;
mod signature;

pub use private_key::SecretKey;
pub use public_key::PublicKey;
pub use signature::Signature;

/// An redjubjub secret key and public key pair.
#[derive(Copy, Clone, Debug)]
pub struct Keypair {
    /// secret key
    pub secret: SecretKey,
    /// public key
    pub public: PublicKey,
}

impl Keypair {
    pub fn new(secret: SecretKey) -> Self {
        let public = secret.to_public_key();
        Self { secret, public }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jub_jub::Fp;
    use rand_core::OsRng;
    use zkstd::behave::Group;

    #[test]
    fn signature_test() {
        for _ in 0..1000 {
            let msg = b"test";
            let wrong_msg = b"tes";
            let randomness = OsRng;

            let priv_key = SecretKey(Fp::random(OsRng));
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

            let priv_key = SecretKey(Fp::random(OsRng));
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
