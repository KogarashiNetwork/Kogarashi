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

use crate::fr::Fr;
use parity_scale_codec::{Decode, Encode};

/// The projective form of coordinate
#[derive(Debug, Clone, Decode, Encode)]
pub(crate) struct Projective {
    x: Fr,
    y: Fr,
    z: Fr,
    is_infinity: bool,
}

/// The projective coordinate addition
/// cost: 12M + 2S + 6A + 1*2
pub(crate) fn add(rhs: Projective, other: Projective) -> Projective {
    // Y1Z2
    let a = rhs.y * other.z;
    // X1Z2
    let b = rhs.x * other.z;
    // Z1Z2
    let c = rhs.z * other.z;

    // Y2*Z1
    let d = other.y * rhs.z;
    // u
    let e = d - b;
    // uu
    let f = e.square();

    // X2*Z1
    let g = other.x * rhs.z;
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

    Projective {
        x: h * n,
        y: p - o,
        z: j * c,
        is_infinity: false,
    }
}

/// The projective coordinate doubling
/// cost: 5M + 6S + 1*a + A + 3*2 + 1*3.
/// a = 0
pub(crate) fn double(rhs: Projective) -> Projective {
    // XX
    let a = rhs.x.square();

    // w
    let b = a.double() + a;
    // s
    let c = rhs.y.double() * rhs.z;
    // ss
    let d = c.square();
    // sss
    let e = d * c;

    // R
    let f = rhs.y * b;
    // RR
    let g = f.square();

    // X1+R
    let h = rhs.x + f;
    // (X1+R)^2
    let i = h.square();
    // B
    let j = i - a - g;
    // h
    let k = c.square() - j.double();

    // w*(B-h)
    let l = b * (j - k);

    Projective {
        x: k * c,
        y: l - g.double(),
        z: e,
        is_infinity: false,
    }
}
