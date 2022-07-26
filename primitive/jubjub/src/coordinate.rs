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

use core::ops::{Add, Mul};

use crate::{fr::Fr, interface::coordinate::Coordinate};
use parity_scale_codec::{Decode, Encode};

/// The projective form of coordinate
#[derive(Debug, Clone, Decode, Encode)]
pub struct Affine {
    x: Fr,
    y: Fr,
    is_infinity: bool,
}

impl Affine {
    pub fn generator() -> Self {
        Self {
            x: Fr::zero(),
            y: Fr::from_hex("0x2").expect("Failed to parse hex"),
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
            z: Fr::one(),
            is_infinity,
        }
    }
}

/// The projective form of coordinate
#[derive(Debug, Clone, Decode, Encode)]
pub struct Projective {
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
        let y1_z2 = self.y * other.z;
        // X1Z2
        let x1_z2 = self.x * other.z;
        // Z1Z2
        let z1_z2 = self.z * other.z;

        // Y2*Z1
        let y2_z1 = other.y * self.z;
        // u
        let u = y2_z1 - y1_z2;
        // uu
        let uu = u.square();

        // X2*Z1
        let x2_z1 = other.x * self.z;
        // v
        let v = x2_z1 - x1_z2;
        // vv
        let vv = v.square();
        // vvv
        let vvv = vv * v;

        // vv * X1 * Z2
        let r = vv * x1_z2;
        // uu*Z1Z2
        let l = uu * z1_z2;
        // A
        let a = l - vvv - r.double();
        // vvv*Y1Z2
        let o = vvv * y1_z2;
        // u*(r-A)
        let p = u * (r - a);

        self.x = v * a;
        self.y = p - o;
        self.z = vvv * z1_z2;
    }

    /// The projective coordinate doubling
    /// cost: 5M + 6S + 1*a + A + 3*2 + 1*3.
    /// a = 0
    pub fn double(&mut self) {
        // XX
        let xx = self.x.square();

        // w
        let w = xx.double() + xx + self.z.square();
        // y1 * z1
        let s = self.y * self.z;
        // 4ss
        let ss_4 = s.double().square();

        // h
        let h = w.square();

        // w*(4B-h)
        let l = -w * h;
        // 4 * yy * ss
        let r = (self.y * s).double().square();

        self.x = h.double() * s;
        self.y = l - r.double();
        self.z = ss_4.double() * s;
    }
}

impl PartialEq for Projective {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Eq for Projective {}

impl Coordinate for Projective {
    fn zero() -> Self {
        Projective {
            x: Fr::zero(),
            y: Fr::zero(),
            z: Fr::zero(),
            is_infinity: false,
        }
    }

    fn one() -> Self {
        // TODO
        Projective {
            x: Fr::zero(),
            y: Fr::zero(),
            z: Fr::zero(),
            is_infinity: true,
        }
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero() && self.z.is_zero()
    }

    fn is_on_curve(&self) -> bool {
        self.y.square()
            == self
                .x
                .square()
                .mul(self.x)
                .add(Fr::from_hex("0x4").unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::interface::coordinate::Coordinate;

    use super::{Affine, Fr, Projective};
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

    // proptest! {
    //     #![proptest_config(ProptestConfig::with_cases(1))]
    //     #[test]
    //     fn test_projective(mut a in arb_cdn(), mut b in arb_cdn()) {
    //         let mut c = a.clone();
    //         let d = b.clone();

    //         a.add(d);
    //         a.double();

    //         c.double();
    //         b.double();
    //         b.add(c);

    //         assert_eq!(a, b);
    //     }
    // }

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

    #[test]
    fn test_on_curve() {
        let a = Projective::zero();
        let b = Projective::from(Affine::generator());
        assert!(!a.is_on_curve());
        assert!(b.is_on_curve());
    }
}
