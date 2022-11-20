// Copyright (C) 2022-2023 Artree (JP) LLC.
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

//! # Elliptic Curve Coordinate System
//!
//! - [`G1Affine`]
//! - [`G1Projective`]
//!
//! ## Overview
//!
//! This coordinate provides the functionalities as following.
//!
//! - Curve addition
//! - Curve doubling
//! - Convert each coordinate system
//!
//! ### Reference
//!
//! We implement coordinate system to refer the following.
//! [Projective coordinates for short Weierstrass curves](https://www.hyperelliptic.org/EFD/g1p/auto-shortw-projective.html)

use crate::fq::Fq;
use crate::fr::Fr;
use zero_crypto::arithmetic::bits_384::*;
use zero_crypto::common::*;
use zero_crypto::dress::curve::*;

/// The projective form of coordinate
#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct G1Projective {
    pub(crate) x: Fq,
    pub(crate) y: Fq,
    pub(crate) z: Fq,
}

const IDENTITY: G1Projective = G1Projective {
    x: Fq::zero(),
    y: Fq::zero(),
    z: Fq::zero(),
};

const GENERATOR: G1Projective = G1Projective {
    x: Fq([
        0x5cb38790fd530c16,
        0x7817fc679976fff5,
        0x154f95c7143ba1c1,
        0xf0ae6acdf3d0e747,
        0xedce6ecc21dbf440,
        0x120177419e0bfb75,
    ]),
    y: Fq([
        0xbaac93d50ce72271,
        0x8c22631a7918fd8e,
        0xdd595f13570725ce,
        0x51ac582950405194,
        0x0e1c8c3fad0059c0,
        0x0bbc3efc5008a26a,
    ]),
    z: Fq::one(),
};

const PARAM_A: Fq = Fq([0, 0, 0, 0, 0, 0]);

const PARAM_B: Fq = Fq([
    0xaa270000000cfff3,
    0x53cc0032fc34000a,
    0x478fe97a6b0a807f,
    0xb1d37ebee6ba24d7,
    0x8ec9733bbf78ab2f,
    0x09d645513d83de7e,
]);

/// The projective form of coordinate
#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct Bls381Affine {
    x: Fq,
    y: Fq,
    is_infinity: bool,
}

curve_operation!(
    Fr,
    Fq,
    PARAM_A,
    PARAM_B,
    Bls381Affine,
    G1Projective,
    GENERATOR,
    IDENTITY
);
