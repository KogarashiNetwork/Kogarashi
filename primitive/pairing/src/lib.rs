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

// #![cfg_attr(not(feature = "std"), no_std)]

use std::print;

use zero_bls12_381::params::{BLS_X, BLS_X_IS_NEGATIVE};
use zero_bls12_381::{Fq12, G1Affine, G1Projective, G2Affine, G2PairingAffine, G2Projective};
use zero_crypto::behave::{G2Pairing, Pairing, PairingRange};
use zero_crypto::common::PrimeField;

// tate pairing with miller algorithm
pub struct TatePairing;

impl Pairing for TatePairing {
    type G1Affine = G1Affine;
    type G2Affine = G2Affine;
    type G1Projective = G1Projective;
    type G2Projective = G2Projective;
    type G2PairngRepr = G2Projective;
    type PairingRange = Fq12;
    const X: u64 = BLS_X;
    const X_IS_NEGATIVE: bool = BLS_X_IS_NEGATIVE;

    fn pairing(g1: Self::G1Affine, g2: Self::G2Affine) -> Self::PairingRange {
        let miller_result = Self::miller_loop(g1, g2);
        println!("miller result {:?}", miller_result);
        match miller_result.final_exp() {
            Some(x) => x,
            None => Self::PairingRange::one(),
        }
    }

    fn miller_loop(g1: Self::G1Affine, g2: Self::G2Affine) -> Self::PairingRange {
        let mut acc = Self::PairingRange::one();
        let mut g2_projective = Self::G2Projective::from(g2);
        let mut found_one = false;

        for i in (0..64).rev().map(|b| (((BLS_X >> 1) >> b) & 1) == 1) {
            if !found_one {
                found_one = i;
                continue;
            }

            let coeffs = g2_projective.double_eval();
            acc = acc.untwist(coeffs, g1);

            if i {
                let coeffs = g2_projective.add_eval(g2);
                acc = acc.untwist(coeffs, g1);
            }

            acc.square_assign();
        }

        let coeffs = g2_projective.double_eval();
        acc = acc.untwist(coeffs, g1);

        if Self::X_IS_NEGATIVE {
            acc = acc.conjugate();
        }
        acc
    }
}
