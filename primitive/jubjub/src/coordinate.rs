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

use core::ops::{Add, Mul, Neg};

use crate::{fr::Fr, interface::coordinate::Coordinate};
use parity_scale_codec::{Decode, Encode};

/// The projective form of coordinate
#[derive(Debug, Clone, Decode, Encode, PartialEq, Eq)]
pub struct Affine {
    x: Fr,
    y: Fr,
    is_infinity: bool,
}

impl Affine {
    pub(crate) fn generator() -> Self {
        Self {
            x: Fr::from_raw([
                0x7c24d812779a3316,
                0x72e38f4ebd4070f3,
                0x03b3fe93f505a6f2,
                0xc4c71e5a4102960,
            ]),
            y: Fr::from_raw([
                0xd2047ef3463de4af,
                0x01ca03640d236cbf,
                0xd3033593ae386e92,
                0xaa87a50921b80ec,
            ]),
            is_infinity: false,
        }
    }
}

impl From<Affine> for Projective {
    fn from(a: Affine) -> Self {
        let Affine { x, y, is_infinity } = a;
        Self {
            x,
            y,
            z: if is_infinity { Fr::zero() } else { Fr::one() },
        }
    }
}

/// The projective form of coordinate
#[derive(Debug, Clone, Decode, Encode)]
pub struct Projective {
    pub(crate) x: Fr,
    pub(crate) y: Fr,
    pub(crate) z: Fr,
}

impl Projective {
    pub fn generator() -> Self {
        Self {
            x: Fr::from_raw([
                0x7c24d812779a3316,
                0x72e38f4ebd4070f3,
                0x03b3fe93f505a6f2,
                0xc4c71e5a4102960,
            ]),
            y: Fr::from_raw([
                0xd2047ef3463de4af,
                0x01ca03640d236cbf,
                0xd3033593ae386e92,
                0xaa87a50921b80ec,
            ]),
            z: Fr::one(),
        }
    }

    pub fn g1() -> Self {
        Self {
            x: Fr::from_raw([
                0x85d3790e014051f1,
                0xaa7f260471bc1570,
                0xeb4ce2141ce5983f,
                0x7a848c10e9359ab,
            ]),
            y: Fr::from_raw([
                0xfab31faba44ff18a,
                0x29c8d2a5308d04ce,
                0xabe251109b820e98,
                0xdb95d587051a4b7,
            ]),
            z: Fr::one(),
        }
    }

    pub fn g2() -> Self {
        Self {
            x: Fr::from_raw([
                0x08f9ee6a9cf52e55,
                0xcd3756a2f35a4943,
                0xaf18ef6d41736986,
                0x54dd209a091bc11,
            ]),
            y: Fr::from_raw([
                0x5642a3b127c5986d,
                0xc9cbd197bdf08e1b,
                0x4e1b56ffdb7f65f9,
                0x110cb52133f089b,
            ]),
            z: Fr::one(),
        }
    }

    /// The projective coordinate addition
    /// cost: 12M + 2S + 6A + 1*2
    pub fn add(&mut self, other: Self) {
        if self.is_identity() {
            *self = other;
        } else if !other.is_identity() {
            let z1z1 = self.z.square();
            let z2z2 = other.z.square();
            let u1 = self.x * z2z2; // 0
            let u2 = other.x * z1z1; // 0
            let s1 = self.y * z2z2 * other.z; // !0
            let s2 = other.y * z1z1 * self.z; // !0

            if u1 == u2 {
                if s1 == s2 {
                    self.double()
                } else {
                    *self = Projective::identity()
                }
            } else {
                let h = u2 - u1;
                let i = h.double().square();
                let j = h * i;
                let r = (s2 - s1).double();
                let v = u1 * i;
                let x3 = r.square() - j - v.double();
                let s1 = (s1 * j).double();
                let y3 = r * (v - x3) - s1;
                let z3 = ((self.z + other.z).square() - z1z1 - z2z2) * h;
                self.x = x3;
                self.y = y3;
                self.z = z3;
            }
        }
    }

    /// The projective coordinate doubling
    /// cost: 1M + 8S + 1*a + 10ADD + 2*2 + 1*3 + 1*8.
    /// a = 0
    pub fn double(&mut self) {
        let xx = self.x.square();
        let yy = self.y.square();
        let yyyy = yy.square();
        let zz = self.z.square();

        let a = self.x + yy;
        let b = a.square() - xx - yyyy;
        let s = b.double();

        let c = xx.double() + xx;
        let d = Fr::zero(); // a = 0
        let m = c + d;
        let e = s.double();
        let t = m.square() - e;

        let f = s - t;
        let l = yyyy.double().double().double();

        let n = self.y * self.z;

        self.x = t;
        self.y = m * f - l;
        self.z = n.square() - yy - zz;
    }

    pub fn neg(&self) -> Self {
        Self {
            x: self.x.neg(),
            y: self.y.neg(),
            z: self.z,
        }
    }

    pub fn to_affine(&self) -> Affine {
        match self.z.invert() {
            Some(inv) => Affine {
                x: self.x * inv,
                y: self.y * inv,
                is_infinity: false,
            },
            None => Affine {
                x: self.x,
                y: self.y,
                is_infinity: true,
            },
        }
    }
}

impl PartialEq for Projective {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Eq for Projective {}

impl Coordinate for Projective {
    fn identity() -> Self {
        Projective {
            x: Fr::zero(),
            y: Fr::zero(),
            z: Fr::zero(),
        }
    }

    fn constant_b() -> Fr {
        Fr::from_raw([4, 0, 0, 0])
    }

    fn is_identity(&self) -> bool {
        self.x.is_zero() && self.y.is_zero() && self.z.is_zero()
    }

    fn is_on_curve(&self) -> bool {
        if self.is_identity() {
            true
        } else {
            self.y.square() == self.x.square().mul(self.x).add(Self::constant_b())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::interface::coordinate::Coordinate;

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
            }
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]
        #[test]
         fn test_projective(mut a in arb_cdn()) {
            let mut b = a.clone();
            let c = a.clone();
            a.double();
            b.add(c);
            assert_eq!(a, b);
        }
    }

    #[test]
    fn test_coordinate_cmp() {
        let a = Projective {
            x: Fr::one(),
            y: Fr::one(),
            z: Fr::one(),
        };
        let b = Projective {
            x: Fr::one(),
            y: Fr::zero(),
            z: Fr::one(),
        };
        assert_ne!(a, b)
    }

    #[test]
    fn test_on_curve() {
        let a = Projective::identity();
        let b = Projective::generator();
        let c = Projective::g1();
        let d = Projective::g2();
        assert!(a.is_on_curve());
        assert!(b.is_on_curve());
        assert!(c.is_on_curve());
        assert!(d.is_on_curve());
    }
}
