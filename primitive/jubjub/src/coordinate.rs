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
pub struct Affine {
    x: Fr,
    y: Fr,
    is_infinity: bool,
}

/// The projective form of coordinate
#[derive(Debug, Clone, Decode, Encode)]
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

    /// The projective coordinate doubling
    /// cost: 5M + 6S + 1*a + A + 3*2 + 1*3.
    /// a = 0
    pub fn double(&mut self) {
        // XX
        let a = self.x.square();

        // w
        let b = a.double() + a;
        // s
        let c = self.y.double() * self.z;
        // ss
        let d = c.square();
        // sss
        let e = d * c;

        // R
        let f = self.y * b;
        // RR
        let g = f.square();

        // X1+R
        let h = self.x + f;
        // (X1+R)^2
        let i = h.square();
        // B
        let j = i - a - g;
        // h
        let k = c.square() - j.double();

        // w*(B-h)
        let l = b * (j - k);

        self.x = k * c;
        self.y = l - g.double();
        self.z = e;
    }
}

impl PartialEq for Projective {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Eq for Projective {}

#[cfg(test)]
mod tests {
    use super::{Fr, Projective};
    use proptest::prelude::*;
    use rand::SeedableRng;
    use rand_xorshift::XorShiftRng;

    prop_compose! {
        fn arb_fr()(bytes in [any::<u8>(); 16]) -> Fr {
            Fr::random(XorShiftRng::from_seed(bytes))
        }
    }

    prop_compose! {
        fn arb_cdn()(x in arb_fr(), y in arb_fr(), z in arb_fr()) -> Projective {
            Projective {
                x,
                y,
                z,
                is_infinity: false
            }
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        #[test]
        fn test_projective(mut a in arb_cdn()) {
            let mut b = a.clone();
            let c = a.clone();
            a.double();
            b.add(c);

            assert_eq!(a, b)
        }
    }

    #[test]
    fn test_coordinate_cmp() {
        let a = Projective {
            x: Fr::one(),
            y: Fr::one(),
            z: Fr::one(),
            is_infinity: false,
        };
        let b = Projective {
            x: Fr::one(),
            y: Fr::zero(),
            z: Fr::one(),
            is_infinity: false,
        };
        assert_ne!(a, b)
    }
}
