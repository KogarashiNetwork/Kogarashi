// Copyright (C) 2022-2023 Invers (JP) INC.
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
//!

use serde::{Deserialize, Serialize};
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};
use zero_bls12_381::Fr;
use zero_crypto::common::*;

// below here, the crate uses [https://github.com/dusk-network/bls12_381](https://github.com/dusk-network/bls12_381) and
// [https://github.com/dusk-network/bls12_381](https://github.com/dusk-network/bls12_381) implementation designed by
// Dusk-Network team and, @str4d and @ebfull

/// The affine form of coordinate
#[derive(Debug, Clone, Copy, Decode, Encode, Serialize, Deserialize)]
pub struct JubjubAffine {
    x: Fr,
    y: Fr,
}

impl JubjubAffine {
    /// Use a fixed generator point.
    /// The point is then reduced according to the prime field. We need only to
    /// state the coordinates, so users can exploit its properties
    /// which are proven by tests, checking:
    /// - It lies on the curve,
    /// - Is of prime order,
    /// - Is not the identity point.
    /// Using:
    ///     x = 0x3fd2814c43ac65a6f1fbf02d0fd6cce62e3ebb21fd6c54ed4df7b7ffec7beaca
    //      y = 0x0000000000000000000000000000000000000000000000000000000000000012
    pub const ADDITIVE_IDENTITY: Self = Self {
        x: Fr::to_mont_form([
            0x4df7b7ffec7beaca,
            0x2e3ebb21fd6c54ed,
            0xf1fbf02d0fd6cce6,
            0x3fd2814c43ac65a6,
        ]),
        y: Fr::to_mont_form([
            0x0000000000000012,
            000000000000000000,
            000000000000000000,
            000000000000,
        ]),
    };

    /// Returns the `x`-coordinate of this point.
    pub const fn get_x(&self) -> Fr {
        self.x
    }

    /// Returns the `y`-coordinate of this point.
    pub const fn get_y(&self) -> Fr {
        self.y
    }

    /// Constructs an JubjubAffine given `x` and `y` without checking
    /// that the point is on the curve.
    pub const fn from_raw_unchecked(x: Fr, y: Fr) -> JubjubAffine {
        JubjubAffine { x, y }
    }
}

impl Neg for JubjubAffine {
    type Output = JubjubAffine;

    /// This computes the negation of a point `P = (x, y)`
    /// as `-P = (-x, y)`.
    #[inline]
    fn neg(self) -> JubjubAffine {
        JubjubAffine {
            x: -self.x,
            y: self.y,
        }
    }
}

impl ConstantTimeEq for JubjubAffine {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.x.ct_eq(&other.x) & self.y.ct_eq(&other.y)
    }
}

impl PartialEq for JubjubAffine {
    fn eq(&self, other: &Self) -> bool {
        self.ct_eq(other).unwrap_u8() == 1
    }
}

impl ConditionallySelectable for JubjubAffine {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        JubjubAffine {
            x: Fr::conditional_select(&a.x, &b.x, choice),
            y: Fr::conditional_select(&a.y, &b.y, choice),
        }
    }
}

/// Use a fixed generator point.
/// The point is then reduced according to the prime field. We need only to
/// state the coordinates, so users can exploit its properties
/// which are proven by tests, checking:
/// - It lies on the curve,
/// - Is of prime order,
/// - Is not the identity point.
/// Using:
///     x = 0x3fd2814c43ac65a6f1fbf02d0fd6cce62e3ebb21fd6c54ed4df7b7ffec7beaca
//      y = 0x0000000000000000000000000000000000000000000000000000000000000012
pub const GENERATOR: JubjubAffine = JubjubAffine {
    x: Fr::to_mont_form([
        0x4df7b7ffec7beaca,
        0x2e3ebb21fd6c54ed,
        0xf1fbf02d0fd6cce6,
        0x3fd2814c43ac65a6,
    ]),
    y: Fr::to_mont_form([
        0x0000000000000012,
        000000000000000000,
        000000000000000000,
        000000000000,
    ]),
};

/// This represents an extended point `(X, Y, Z, T1, T2)`
/// with `Z` nonzero, corresponding to the affine point
/// `(X/Z, Y/Z)`. We always have `T1 * T2 = XY/Z`.
///
/// You can do the following things with a point in this
/// form:
///
/// * Convert it into a point in the affine form.
/// * Add it to an `JubJubExtended`, `AffineNielsPoint` or `ExtendedNielsPoint`.
/// * Double it using `double()`.
/// * Compare it with another extended point using `PartialEq` or `ct_eq()`.
#[derive(Clone, Debug, Copy)]
pub struct JubJubExtended {
    x: Fr,
    y: Fr,
    z: Fr,
    t1: Fr,
    t2: Fr,
}

impl JubJubExtended {
    pub const ADDITIVE_GENERATOR: Self = Self {
        x: Fr::zero(),
        y: Fr::one(),
        z: Fr::one(),
        t1: Fr::zero(),
        t2: Fr::zero(),
    };

    /// Constructs an extended point (with `Z = 1`) from
    /// an affine point using the map `(x, y) => (x, y, 1, x, y)`.
    pub const fn from_affine(affine: JubjubAffine) -> Self {
        Self {
            x: affine.x,
            y: affine.y,
            z: Fr::one(),
            t1: affine.x,
            t2: affine.y,
        }
    }

    /// Returns the `x`-coordinate of this point.
    pub const fn get_x(&self) -> Fr {
        self.x
    }

    /// Returns the `y`-coordinate of this point.
    pub const fn get_y(&self) -> Fr {
        self.y
    }

    /// Returns the `z`-coordinate of this point.
    pub const fn get_z(&self) -> Fr {
        self.z
    }

    /// Computes the doubling of a point more efficiently than a point can
    /// be added to itself.
    pub fn double(&self) -> Self {
        // Doubling is more efficient (three multiplications, four squarings)
        // when we work within the projective coordinate space (U:Z, V:Z). We
        // rely on the most efficient formula, "dbl-2008-bbjlp", as described
        // in Section 6 of "Twisted Edwards Curves" by Bernstein et al.
        //
        // See <https://hyperelliptic.org/EFD/g1p/auto-twisted-projective.html#doubling-dbl-2008-bbjlp>
        // for more information.
        //
        // We differ from the literature in that we use (x, y) rather than
        // (x, y) coordinates. We also have the constant `a = -1` implied. Let
        // us rewrite the procedure of doubling (x, y, z) to produce (X, Y, Z)
        // as follows:
        //
        // B = (x + y)^2
        // C = x^2
        // D = y^2
        // F = D - C
        // H = 2 * z^2
        // J = F - H
        // X = (B - C - D) * J
        // Y = F * (- C - D)
        // Z = F * J
        //
        // If we compute K = D + C, we can rewrite this:
        //
        // B = (x + y)^2
        // C = x^2
        // D = y^2
        // F = D - C
        // K = D + C
        // H = 2 * z^2
        // J = F - H
        // X = (B - K) * J
        // Y = F * (-K)
        // Z = F * J
        //
        // In order to avoid the unnecessary negation of K,
        // we will negate J, transforming the result into
        // an equivalent point with a negated z-coordinate.
        //
        // B = (x + y)^2
        // C = x^2
        // D = y^2
        // F = D - C
        // K = D + C
        // H = 2 * z^2
        // J = H - F
        // X = (B - K) * J
        // Y = F * K
        // Z = F * J
        //
        // Let us rename some variables to simplify:
        //
        // XY2 = (x + y)^2
        // XX = x^2
        // YY = y^2
        // YYmXX = YY - XX
        // YYpXX = YY + XX
        // ZZ2 = 2 * z^2
        // J = ZZ2 - YYmXX
        // X = (XY2 - YYpXX) * J
        // Y = YYmXX * YYXX
        // Z = YYmXX * J
        //
        // We wish to obtain two factors of T = XY / Z.
        //
        // XY / Z
        // =
        // (XY2 - YYpXX) * (ZZ2 - VVmUU) * YYmXX * YYpXX / YYmXX / (ZZ2 - YYmXX)
        // =
        // (XY2 - YYpXX) * YYmXX * YYpXX / YYmXX
        // =
        // (XY2 - YYpXX) * YYpXX
        //
        // and so we have that T1 = (XY2 - YYpXX) and T2 = YYpXX.

        let xx = self.x.square();
        let yy = self.y.square();
        let zz2 = self.z.square().double();
        let xy2 = (self.x + self.y).square();
        let yy_plus_xx = yy + xx;
        let yy_minus_xx = yy - xx;

        // The remaining arithmetic is exactly the process of converting
        // from a completed point to an extended point.
        CompletedPoint {
            x: xy2 - yy_plus_xx,
            y: yy_plus_xx,
            z: yy_minus_xx,
            t: zz2 - yy_minus_xx,
        }
        .into_extended()
    }
}

/// This is a "completed" point produced during a point doubling or
/// addition routine. These points exist in the `(X:Z, Y:T)` model
/// of the curve. This is not exposed in the API because it is
/// an implementation detail.
struct CompletedPoint {
    x: Fr,
    y: Fr,
    z: Fr,
    t: Fr,
}

impl CompletedPoint {
    /// This converts a completed point into an extended point by
    /// homogenizing:
    ///
    /// (x/z, y/t) = (x/z * t/t, y/t * z/z) = (xt/zt, yz/zt)
    ///
    /// The resulting T coordinate is xtyz/zt = xy, and so
    /// T1 = x, T2 = y, without loss of generality.
    #[inline]
    fn into_extended(self) -> JubJubExtended {
        JubJubExtended {
            x: &self.x * &self.t,
            y: &self.y * &self.z,
            z: &self.z * &self.t,
            t1: self.x,
            t2: self.y,
        }
    }
}

/// This takes a mutable slice of `JubJubExtended`s and "normalizes" them using
/// only a single inversion for the entire batch. This normalization results in
/// all of the points having a Z-coordinate of one. Further, an iterator is
/// returned which can be used to obtain `JubjubAffine`s for each element in the
/// slice.
///
/// This costs 5 multiplications per element, and a field inversion.
pub fn batch_normalize<'a>(y: &'a mut [JubJubExtended]) -> impl Iterator<Item = JubjubAffine> + 'a {
    let mut acc = Fr::one();
    for p in y.iter_mut() {
        // We use the `t1` field of `JubJubExtended` to store the product
        // of previous z-coordinates seen.
        p.t1 = acc;
        acc *= p.z;
    }

    // This is the inverse, as all z-coordinates are nonzero.
    acc = acc.invert().unwrap();

    for p in y.iter_mut().rev() {
        let mut q = *p;

        // Compute tmp = 1/z
        let tmp = q.t1 * acc;

        // Cancel out z-coordinate in denominator of `acc`
        acc *= q.z;

        // Set the coordinates to the correct value
        q.x *= tmp; // Multiply by 1/z
        q.y *= tmp; // Multiply by 1/z
        q.z = Fr::one(); // z-coordinate is now one
        q.t1 = q.x;
        q.t2 = q.y;

        *p = q;
    }

    // All extended points are now normalized, but the type
    // doesn't encode this fact. Let us offer affine points
    // to the caller.

    y.iter().map(|p| JubjubAffine { x: p.x, y: p.y })
}

impl From<JubjubAffine> for JubJubExtended {
    fn from(affine: JubjubAffine) -> JubJubExtended {
        Self::from_affine(affine)
    }
}

impl JubjubAffine {
    /// Constructs an JubjubAffine given `x` and `y` without checking
    /// that the point is on the curve.
    pub const fn to_mont_form_unchecked(x: Fr, y: Fr) -> JubjubAffine {
        JubjubAffine { x, y }
    }
}
