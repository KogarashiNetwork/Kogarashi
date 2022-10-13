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

use crate::coordinate::Projective;
use crate::fr::Fr;
use crate::interface::coordinate::Coordinate;

/// The projective coordinate addition
/// cost: 12M + 2S + 6A + 1*2
pub(crate) fn add(rhs: Projective, other: Projective) -> Projective {
    if rhs.is_identity() {
        return other;
    } else if !other.is_identity() {
        let z1z1 = rhs.z.square();
        let z2z2 = other.z.square();
        let u1 = rhs.x * z2z2; // 0
        let u2 = other.x * z1z1; // 0
        let s1 = rhs.y * z2z2 * other.z; // !0
        let s2 = other.y * z1z1 * rhs.z; // !0

        if u1 == u2 {
            if s1 == s2 {
                return double(rhs);
            } else {
                return Projective::identity();
            }
        } else {
            let h = u2 - u1;
            let i = h.double().square();
            let j = h * i;
            let r = (s2 - s1).double();
            let v = u1 * i;
            let x3 = r.square() - j - v.double();
            let s1 = (s1 * j).double();

            return Projective {
                x: x3,
                y: r * (v - x3) - s1,
                z: ((rhs.z + other.z).square() - z1z1 - z2z2) * h,
            };
        }
    }

    rhs
}

/// The projective coordinate doubling
/// cost: 5M + 6S + 1*a + A + 3*2 + 1*3.
/// a = 0
pub(crate) fn double(rhs: Projective) -> Projective {
    let xx = rhs.x.square();
    let yy = rhs.y.square();
    let yyyy = yy.square();
    let zz = rhs.z.square();

    let a = rhs.x + yy;
    let b = a.square() - xx - yyyy;
    let s = b.double();

    let c = xx.double() + xx;
    let d = Fr::zero(); // a = 0
    let m = c + d;
    let e = s.double();
    let t = m.square() - e;

    let f = s - t;
    let l = yyyy.double().double().double();

    let n = rhs.y * rhs.z;

    Projective {
        x: t,
        y: m * f - l,
        z: n.square() - yy - zz,
    }
}
