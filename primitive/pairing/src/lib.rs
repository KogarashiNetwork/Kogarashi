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

use zero_bls12_381::{Fq, Fq12, Fq2, Fq6, G1Affine, G1Projective, G2Affine, G2Projective};
use zero_crypto::behave::Pairing;

// tate pairing with miller algorithm
pub struct TatePairing {}

impl Pairing for TatePairing {
    type G1Affine = G1Affine;

    type G2Affine = G2Affine;

    type G1Projective = G1Projective;

    type G2Projective = G2Projective;

    type PairingRange = Fq12;

    fn pairing(g1: Self::G1Affine, g2: Self::G2Affine) -> Self::PairingRange {
        pairing(g1, g2)
    }

    fn miller_loop(
        g2_affine: Self::G2Affine,
        g2_projective: Self::G2Projective,
        poly: Self::PairingRange,
    ) -> Self::PairingRange {
        miller_loop(g2_affine, g2_projective, poly)
    }
}

fn pairing(g1: G1Affine, g2: G2Affine) -> Fq12 {
    Fq12::zero()
}

fn miller_loop(g2_affine: G2Affine, g2_projective: G2Projective, poly: Fq12) -> Fq12 {
    Fq12::zero()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
