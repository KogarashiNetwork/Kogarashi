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
//! - [`Affine`]
//! - [`Projective`]
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

use crate::entity::Fr;

/// The affine form of coordinate
#[derive(Debug)]
pub(crate) struct Affine {
    x: Fr,
    y: Fr,
}

impl Affine {
    pub fn add(&self, other: Self) {}

    pub fn double(&self) {}
}

/// The projective form of coordinate
pub(crate) struct Projective {
    x: Fr,
    y: Fr,
    z: Fr,
    is_infinity: bool,
}

impl Projective {
    /// The projective coordinate addition
    /// cost: 12M + 2S + 6A + 1*2
    pub fn add(&mut self, other: Self) {
        // Y1Z2
        let a = self.y * other.z;
        // X1Z2
        let b = self.x * other.z;
        // Z1Z2
        let c = self.z * other.z;

        // Y2*Z1
        let d = other.y * self.z;
        // u
        let e = d - b;
        // uu
        let f = e.square();

        // X2*Z1
        let g = other.x * self.z;
        // v
        let h = g - a;
        // vv
        let i = h.square();
        // vvv
        let j = i * h;

        // R
        let k = i * a;
        // uu*Z1Z2
        let l = f * c;
        // 2*R
        let m = k.double();
        // A
        let n = l - j - m;
        // vvv*Y1Z2
        let o = j * b;
        // u*(R-A)
        let p = e * (k - n);

        self.x = h * n;
        self.y = p - o;
        self.z = j * c;
    }

    pub fn double(&mut self, other: Self) {}
}
