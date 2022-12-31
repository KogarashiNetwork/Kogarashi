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

#![cfg_attr(not(feature = "std"), no_std)]

use zero_bls12_381::params::{BLS_X, BLS_X_IS_NEGATIVE};
use zero_bls12_381::{Fq12, G1Affine, G1Projective, G2Affine, G2PairingAffine, G2Projective};
use zero_crypto::behave::{Pairing, PairingRange};
use zero_crypto::common::PrimeField;

// tate pairing with miller algorithm
pub struct TatePairing;

impl Pairing for TatePairing {
    type G1Affine = G1Affine;
    type G2Affine = G2Affine;
    type G1Projective = G1Projective;
    type G2Projective = G2Projective;
    type G2PairngRepr = G2PairingAffine;
    type PairingRange = Fq12;
    const X: u64 = BLS_X;
    const X_ISNEGATIVE: bool = BLS_X_IS_NEGATIVE;

    fn pairing(g1: Self::G1Affine, g2: Self::G2PairngRepr) -> Self::PairingRange {
        Self::miller_loop(g1, g2).final_exp().unwrap()
    }

    fn miller_loop(g1: Self::G1Affine, g2: Self::G2PairngRepr) -> Self::PairingRange {
        let mut acc = Self::PairingRange::one();

        let mut found_one = false;
        for i in (0..64).rev().map(|b| (((BLS_X >> 1) >> b) & 1) == 1) {
            if !found_one {
                found_one = i;
                continue;
            }

            for coeff in g2.coeffs.iter() {
                acc = acc.untwist(*coeff, g1);
            }

            if i {
                for coeff in g2.coeffs.iter() {
                    acc = acc.untwist(*coeff, g1);
                }
            }

            acc = acc.square();
        }

        for coeff in g2.coeffs.iter() {
            acc = acc.untwist(*coeff, g1);
        }

        if Self::X_ISNEGATIVE {
            acc.conjugate()
        } else {
            acc
        }
    }
}

#[cfg(test)]
mod pairing_tests {
    use super::*;
    use rand_core::OsRng;
    use zero_bls12_381::Fr;
    use zero_crypto::common::Group;

    fn arb_fr() -> Fr {
        Fr::random(OsRng)
    }

    #[test]
    fn generator_pairing_test() {
        let g1 = G1Affine::GENERATOR;
        let g2 = G2PairingAffine::from(G2Projective::GENERATOR);

        assert_eq!(Fq12::one(), TatePairing::pairing(g1, g2));
    }

    #[test]
    fn pairing_test() {
        let g1 = G1Affine::GENERATOR;
        let g2 = G2Affine::GENERATOR;

        let a = arb_fr();
        let b = arb_fr();
        let c = a * b;

        let g = g1 * a;
        let h = g2 * b;
    }

    #[test]
    fn final_exp_test() {
        assert_eq!(Fq12::one().final_exp().unwrap(), Fq12::one());
    }
}
