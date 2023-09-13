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

use bls_12_381::params::{BLS_X, BLS_X_IS_NEGATIVE};
use bls_12_381::{Fq12, Fr, G1Affine, G1Projective, G2Affine, G2PairingAffine, G2Projective, Gt};
use jub_jub::{Fp, JubjubAffine, JubjubExtended};
use zkstd::common::Vec;
use zkstd::common::*;

/// Tate pairing struct holds necessary components for pairing.
/// `pairing` function takes G1 and G2 group elements and output
/// GT target group element.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Encode, Decode, Copy)]
pub struct TatePairing;

impl Pairing for TatePairing {
    type G1Affine = G1Affine;
    type G2Affine = G2Affine;
    type G1Projective = G1Projective;
    type G2Projective = G2Projective;
    type JubjubAffine = JubjubAffine;
    type JubjubExtended = JubjubExtended;
    type G2PairngRepr = G2PairingAffine;
    type PairingRange = Fq12;
    type Gt = Gt;
    type ScalarField = Fr;
    type JubjubScalar = Fp;
    const X: u64 = BLS_X;
    const X_IS_NEGATIVE: bool = BLS_X_IS_NEGATIVE;

    fn pairing(g1: Self::G1Affine, g2: Self::G2Affine) -> Self::Gt {
        Self::miller_loop(g1, g2).final_exp()
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

            acc = acc.untwist(g2_projective.double_eval(), g1);

            if i {
                acc = acc.untwist(g2_projective.add_eval(g2), g1);
            }

            acc.square_assign();
        }

        acc = acc.untwist(g2_projective.double_eval(), g1);

        if Self::X_IS_NEGATIVE {
            acc.conjugate()
        } else {
            acc
        }
    }

    fn multi_miller_loop(pairs: &[(Self::G1Affine, Self::G2PairngRepr)]) -> Self::PairingRange {
        let pairs = pairs
            .iter()
            .filter(|(a, b)| !a.is_identity() && !b.is_identity())
            .collect::<Vec<_>>();
        let mut acc = Self::PairingRange::one();
        let mut counter = 0;
        let mut found_one = false;

        for i in (0..64).rev().map(|b| (((BLS_X >> 1) >> b) & 1) == 1) {
            if !found_one {
                found_one = i;
                continue;
            }

            for (g1, g2) in pairs.iter() {
                acc = acc.untwist(g2.coeffs[counter], *g1);
            }
            counter += 1;

            if i {
                for (g1, g2) in pairs.iter() {
                    acc = acc.untwist(g2.coeffs[counter], *g1);
                }
                counter += 1;
            }

            acc.square_assign();
        }

        for (g1, g2) in pairs {
            acc = acc.untwist(g2.coeffs[counter], *g1);
        }

        if Self::X_IS_NEGATIVE {
            acc.conjugate()
        } else {
            acc
        }
    }
}
