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
use crate::interface::coordinate::Coordinate;
use zero_crypto::behave::*;

/// The projective coordinate addition
/// cost: 12M + 2S + 6A + 1*2
pub(crate) fn add(lhs: Projective, rhs: Projective) -> Projective {
    if lhs.is_identity() {
        return rhs;
    } else if !rhs.is_identity() {
        let s1 = lhs.y * rhs.z;
        let s2 = rhs.y * lhs.z;
        let u1 = lhs.x * rhs.z;
        let u2 = rhs.x * lhs.z;

        if u1 == u2 {
            if s1 == s2 {
                return double(lhs);
            } else {
                return Projective::identity();
            }
        } else {
            let s = s1 - s2;
            let u = u1 - u2;
            let uu = u.square();
            let v = lhs.z * rhs.z;
            let w = s.square() * v - uu * (u1 + u2);
            let uuu = uu * u;
            return Projective {
                x: u * w,
                y: s * (u1 * uu - w) - s1 * uuu,
                z: uuu * v,
            };
        }
    }
    lhs
}

/// The projective coordinate doubling
/// cost: 5M + 6S + 1*a + A + 3*2 + 1*3.
/// a = 0, b = 4
pub(crate) fn double(point: Projective) -> Projective {
    if point.is_identity() || point.y.is_zero() {
        Projective::identity()
    } else {
        let xx = point.x.square();
        let t = xx.double() + xx;
        let u = (point.y * point.z).double();
        let v = (u * point.x * point.y).double();
        let w = t.square() - v.double();
        Projective {
            x: u * w,
            y: t * (v - w) - (u.square() * point.y.square()).double(),
            z: u.square() * u,
        }
    }
}
