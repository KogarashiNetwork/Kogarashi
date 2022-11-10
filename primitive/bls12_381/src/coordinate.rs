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
//! - [`Bls381Affine`]
//! - [`Bls381Projective`]
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

use crate::fp::Fp;
use zero_crypto::arithmetic::coordinate::bits_384::projective::*;
use zero_crypto::behave::*;
use zero_crypto::common::*;
use zero_crypto::dress::basic::curve::*;
use zero_crypto::dress::curve::*;

/// The projective form of coordinate
#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct Bls381Projective {
    pub(crate) x: Fp,
    pub(crate) y: Fp,
    pub(crate) z: Fp,
}

const IDENTITY: Bls381Projective = Bls381Projective {
    x: Fp::zero(),
    y: Fp::zero(),
    z: Fp::zero(),
};

const GENERATOR: Bls381Projective = Bls381Projective {
    x: Fp([
        0x5cb3_8790_fd53_0c16,
        0x7817_fc67_9976_fff5,
        0x154f_95c7_143b_a1c1,
        0xf0ae_6acd_f3d0_e747,
        0xedce_6ecc_21db_f440,
        0x1201_7741_9e0b_fb75,
    ]),
    y: Fp([
        0xbaac_93d5_0ce7_2271,
        0x8c22_631a_7918_fd8e,
        0xdd59_5f13_5707_25ce,
        0x51ac_5829_5040_5194,
        0x0e1c_8c3f_ad00_59c0,
        0x0bbc_3efc_5008_a26a,
    ]),
    z: Fp::one(),
};

const PARAM_A: Fp = Fp([0, 0, 0, 0, 0, 0]);

const PARAM_B: Fp = Fp([
    0xaa27_0000_000c_fff3,
    0x53cc_0032_fc34_000a,
    0x478f_e97a_6b0a_807f,
    0xb1d3_7ebe_e6ba_24d7,
    0x8ec9_733b_bf78_ab2f,
    0x09d6_4551_3d83_de7e,
]);

pub struct Bls381Curve {}

/// The projective form of coordinate
#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct Bls381Affine {
    x: Fp,
    y: Fp,
    is_infinity: bool,
}

type Mont = [u64; 12];

type Bits = [u8; 384];

curve_operation!(
    Bls381Curve,
    Fp,
    PARAM_A,
    PARAM_B,
    Bls381Affine,
    Bls381Projective,
    GENERATOR,
    IDENTITY,
    Mont,
    Bits
);
