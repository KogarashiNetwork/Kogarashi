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
//! - [`JubjubAffine`]
//! - [`JubjubProjective`]
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
use zero_crypto::behave::*;
use zero_crypto::common::*;
use zero_crypto::dress::basic::curve::*;
use zero_crypto::dress::curve::*;

const IDENTITY: JubjubProjective = JubjubProjective {
    x: Fr::zero(),
    y: Fr::zero(),
    z: Fr::zero(),
};

const GENERATOR: JubjubProjective = JubjubProjective {
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
};

const PARAM_A: Fr = Fr::zero();

const PARAM_B: Fr = Fr::from_raw([4, 0, 0, 0]);

pub struct JubJubCurve {}

/// The projective form of coordinate
#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct JubjubAffine {
    x: Fr,
    y: Fr,
    is_infinity: bool,
}

/// The projective form of coordinate
#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct JubjubProjective {
    pub(crate) x: Fr,
    pub(crate) y: Fr,
    pub(crate) z: Fr,
}

curve_operation!(
    JubJubCurve,
    Fr,
    PARAM_A,
    PARAM_B,
    JubjubAffine,
    JubjubProjective,
    GENERATOR,
    IDENTITY
);

#[cfg(test)]
mod tests {
    use super::{Fr, JubjubProjective};
    use proptest::prelude::*;
    use rand::SeedableRng;
    use rand_xorshift::XorShiftRng;
    use zero_crypto::behave::Projective;

    prop_compose! {
        fn arb_fr()(bytes in [any::<u8>(); 16]) -> Fr {
            Fr::random(XorShiftRng::from_seed(bytes))
        }
    }

    prop_compose! {
        fn arb_cdn()(k in arb_fr()) -> JubjubProjective {
            JubjubProjective::GENERATOR * k
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]
        #[test]
         fn test_projective(mut a in arb_cdn(), mut b in arb_cdn(), mut c in arb_cdn()) {
            let mut base_for_neg = a.clone();
            let _b = b.clone();
            let mut a1 = a.clone();
            assert!(a.is_on_curve());
            assert!(b.is_on_curve());
            assert!(c.is_on_curve());
            a1.add(b.clone());
            a1.add(c.clone());
            c.add(a.clone());
            c.add(b.clone());
            assert_eq!(a1, c); // A + B + C == C + A + B
            let mut x = a.clone();
            let y = a.clone();
            a.double();
            x.add(y);
            assert_eq!(a, x); // A * A = A + A
            base_for_neg.add(base_for_neg.neg());
            assert_eq!(base_for_neg, JubjubProjective::IDENTITY); // A + (-A) = e

            b.add(JubjubProjective::IDENTITY);
            assert_eq!(b, _b); // X + e == X
        }
    }

    #[test]
    fn test_identity() {
        let mut iden = JubjubProjective::IDENTITY;
        assert!(iden.is_on_curve());
        iden.double();
        assert_eq!(iden, JubjubProjective::IDENTITY); // e * e = e
    }

    #[test]
    fn test_coordinate_cmp() {
        let a = JubjubProjective {
            x: Fr::one(),
            y: Fr::one(),
            z: Fr::one(),
        };
        let b = JubjubProjective {
            x: Fr::one(),
            y: Fr::zero(),
            z: Fr::one(),
        };
        assert_ne!(a, b)
    }

    #[test]
    fn test_on_curve() {
        let gen = Projective::generator();
        let mut g1 = Projective::g1();
        let mut g2 = Projective::g2();

        assert!(gen.is_on_curve());
        assert!(g1.is_on_curve());
        assert!(g2.is_on_curve());

        g1.double();
        g2.add(Projective::g1());
        assert!(g1.is_on_curve());
        assert!(g2.is_on_curve());
    }
}
