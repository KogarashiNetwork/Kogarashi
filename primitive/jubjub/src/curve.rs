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

use crate::fp::Fp;
use dusk_bytes::{Error as BytesError, Serializable};
use serde::{Deserialize, Serialize};
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};
use zero_bls12_381::Fr;
use zero_crypto::common::*;

// below here, the crate uses [https://github.com/dusk-network/bls12_381](https://github.com/dusk-network/bls12_381) and
// [https://github.com/dusk-network/bls12_381](https://github.com/dusk-network/bls12_381) implementation designed by
// Dusk-Network team and, @str4d and @ebfull

macro_rules! impl_add_binop_specify_output {
    ($lhs:ident, $rhs:ident, $output:ident) => {
        impl<'b> Add<&'b $rhs> for $lhs {
            type Output = $output;

            #[inline]
            fn add(self, rhs: &'b $rhs) -> $output {
                &self + rhs
            }
        }

        impl<'a> Add<$rhs> for &'a $lhs {
            type Output = $output;

            #[inline]
            fn add(self, rhs: $rhs) -> $output {
                self + &rhs
            }
        }

        impl Add<$rhs> for $lhs {
            type Output = $output;

            #[inline]
            fn add(self, rhs: $rhs) -> $output {
                &self + &rhs
            }
        }
    };
}

macro_rules! impl_sub_binop_specify_output {
    ($lhs:ident, $rhs:ident, $output:ident) => {
        impl<'b> Sub<&'b $rhs> for $lhs {
            type Output = $output;

            #[inline]
            fn sub(self, rhs: &'b $rhs) -> $output {
                &self - rhs
            }
        }

        impl<'a> Sub<$rhs> for &'a $lhs {
            type Output = $output;

            #[inline]
            fn sub(self, rhs: $rhs) -> $output {
                self - &rhs
            }
        }

        impl Sub<$rhs> for $lhs {
            type Output = $output;

            #[inline]
            fn sub(self, rhs: $rhs) -> $output {
                &self - &rhs
            }
        }
    };
}

macro_rules! impl_binops_additive_specify_output {
    ($lhs:ident, $rhs:ident, $output:ident) => {
        impl_add_binop_specify_output!($lhs, $rhs, $output);
        impl_sub_binop_specify_output!($lhs, $rhs, $output);
    };
}

macro_rules! impl_binops_multiplicative_mixed {
    ($lhs:ident, $rhs:ident, $output:ident) => {
        impl<'b> Mul<&'b $rhs> for $lhs {
            type Output = $output;

            #[inline]
            fn mul(self, rhs: &'b $rhs) -> $output {
                &self * rhs
            }
        }

        impl<'a> Mul<$rhs> for &'a $lhs {
            type Output = $output;

            #[inline]
            fn mul(self, rhs: $rhs) -> $output {
                self * &rhs
            }
        }

        impl Mul<$rhs> for $lhs {
            type Output = $output;

            #[inline]
            fn mul(self, rhs: $rhs) -> $output {
                &self * &rhs
            }
        }
    };
}

macro_rules! impl_binops_additive {
    ($lhs:ident, $rhs:ident) => {
        impl_binops_additive_specify_output!($lhs, $rhs, $lhs);

        impl SubAssign<$rhs> for $lhs {
            #[inline]
            fn sub_assign(&mut self, rhs: $rhs) {
                *self = &*self - &rhs;
            }
        }

        impl AddAssign<$rhs> for $lhs {
            #[inline]
            fn add_assign(&mut self, rhs: $rhs) {
                *self = &*self + &rhs;
            }
        }

        impl<'b> SubAssign<&'b $rhs> for $lhs {
            #[inline]
            fn sub_assign(&mut self, rhs: &'b $rhs) {
                *self = &*self - rhs;
            }
        }

        impl<'b> AddAssign<&'b $rhs> for $lhs {
            #[inline]
            fn add_assign(&mut self, rhs: &'b $rhs) {
                *self = &*self + rhs;
            }
        }
    };
}

macro_rules! impl_binops_multiplicative {
    ($lhs:ident, $rhs:ident) => {
        impl_binops_multiplicative_mixed!($lhs, $rhs, $lhs);

        impl MulAssign<$rhs> for $lhs {
            #[inline]
            fn mul_assign(&mut self, rhs: $rhs) {
                *self = &*self * &rhs;
            }
        }

        impl<'b> MulAssign<&'b $rhs> for $lhs {
            #[inline]
            fn mul_assign(&mut self, rhs: &'b $rhs) {
                *self = &*self * rhs;
            }
        }
    };
}

const FR_MODULUS_BYTES: [u8; 32] = [
    183, 44, 247, 214, 94, 14, 151, 208, 130, 16, 200, 204, 147, 32, 104, 166, 0, 59, 52, 1, 1, 59,
    103, 6, 169, 175, 51, 101, 234, 180, 125, 14,
];

/// This represents a Jubjub point in the affine `(x, y)`
/// coordinates.
#[derive(Clone, Copy, Debug, Encode, Decode, Deserialize, Serialize)]
pub struct JubJubAffine {
    x: Fr,
    y: Fr,
}

impl Neg for JubJubAffine {
    type Output = JubJubAffine;

    /// This computes the negation of a point `P = (x, y)`
    /// as `-P = (-x, y)`.
    #[inline]
    fn neg(self) -> JubJubAffine {
        JubJubAffine {
            x: -self.x,
            y: self.y,
        }
    }
}

impl ConstantTimeEq for JubJubAffine {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.x.ct_eq(&other.x) & self.y.ct_eq(&other.y)
    }
}

impl PartialEq for JubJubAffine {
    fn eq(&self, other: &Self) -> bool {
        self.ct_eq(other).unwrap_u8() == 1
    }
}

impl ConditionallySelectable for JubJubAffine {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        JubJubAffine {
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
pub const GENERATOR: JubJubAffine = JubJubAffine {
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

/// [`GENERATOR`] in [`JubJubExtended`] form
pub const GENERATOR_EXTENDED: JubJubExtended = JubJubExtended {
    x: GENERATOR.x,
    y: GENERATOR.y,
    z: Fr::one(),
    t1: GENERATOR.x,
    t2: GENERATOR.y,
};

/// GENERATOR NUMS which is obtained following the specs in:
/// https://app.gitbook.com/@dusk-network/s/specs/specifications/poseidon/pedersen-commitment-scheme
/// The counter = 18 and the hash function used to compute it was blake2b
/// Using:
///     x = 0x5e67b8f316f414f7bd9514c773fd4456931e316a39fe4541921710179df76377
//      y = 0x43d80eb3b2f3eb1b7b162dbeeb3b34fd9949ba0f82a5507a6705b707162e3ef8
pub const GENERATOR_NUMS: JubJubAffine = JubJubAffine {
    x: Fr::to_mont_form([
        0x921710179df76377,
        0x931e316a39fe4541,
        0xbd9514c773fd4456,
        0x5e67b8f316f414f7,
    ]),
    y: Fr::to_mont_form([
        0x6705b707162e3ef8,
        0x9949ba0f82a5507a,
        0x7b162dbeeb3b34fd,
        0x43d80eb3b2f3eb1b,
    ]),
};

/// [`GENERATOR_NUMS`] in [`JubJubExtended`] form
pub const GENERATOR_NUMS_EXTENDED: JubJubExtended = JubJubExtended {
    x: GENERATOR_NUMS.x,
    y: GENERATOR_NUMS.y,
    z: Fr::one(),
    t1: GENERATOR_NUMS.x,
    t2: GENERATOR_NUMS.y,
};

// 202, 234, 123, 236, 255, 183, 247, 77, 237, 84, 108, 253, 33, 187, 62, 46,
// 230, 204, 214,15, 45, 240, 251, 241, 166, 101, 172, 67, 76, 129, 210, 63,

// 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
// 0, 0, 0, 0, 0, 0, 0,

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
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "canon", derive(Canon))]
pub struct JubJubExtended {
    x: Fr,
    y: Fr,
    z: Fr,
    t1: Fr,
    t2: Fr,
}

impl ConstantTimeEq for JubJubExtended {
    fn ct_eq(&self, other: &Self) -> Choice {
        // (x/z, y/z) = (x'/z', y'/z') is implied by
        //      (xz'z = x'z'z) and
        //      (yz'z = y'z'z)
        // as z and z' are always nonzero.

        (&self.x * &other.z).ct_eq(&(&other.x * &self.z))
            & (&self.y * &other.z).ct_eq(&(&other.y * &self.z))
    }
}

impl ConditionallySelectable for JubJubExtended {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        JubJubExtended {
            x: Fr::conditional_select(&a.x, &b.x, choice),
            y: Fr::conditional_select(&a.y, &b.y, choice),
            z: Fr::conditional_select(&a.z, &b.z, choice),
            t1: Fr::conditional_select(&a.t1, &b.t1, choice),
            t2: Fr::conditional_select(&a.t2, &b.t2, choice),
        }
    }
}

impl PartialEq for JubJubExtended {
    fn eq(&self, other: &Self) -> bool {
        self.ct_eq(other).unwrap_u8() == 1
    }
}

impl Neg for JubJubExtended {
    type Output = JubJubExtended;

    /// Computes the negation of a point `P = (X, Y, Z, T)`
    /// as `-P = (-X, Y, Z, -T1, T2)`. The choice of `T1`
    /// is made without loss of generality.
    #[inline]
    fn neg(self) -> JubJubExtended {
        JubJubExtended {
            x: -self.x,
            y: self.y,
            z: self.z,
            t1: -self.t1,
            t2: self.t2,
        }
    }
}

impl From<JubJubAffine> for JubJubExtended {
    fn from(affine: JubJubAffine) -> JubJubExtended {
        Self::from_affine(affine)
    }
}

impl<'a> From<&'a JubJubExtended> for JubJubAffine {
    /// Constructs an affine point from an extended point
    /// using the map `(X, Y, Z, T1, T2) => (XZ, Y/Z)`
    /// as Z is always nonzero. **This requires a field inversion
    /// and so it is recommended to perform these in a batch
    /// using [`batch_normalize`](crate::batch_normalize) instead.**
    fn from(extended: &'a JubJubExtended) -> JubJubAffine {
        // Z coordinate is always nonzero, so this is
        // its inverse.
        let zinv = extended.z.invert().unwrap();

        JubJubAffine {
            x: extended.x * &zinv,
            y: extended.y * &zinv,
        }
    }
}

impl From<JubJubExtended> for JubJubAffine {
    fn from(extended: JubJubExtended) -> JubJubAffine {
        JubJubAffine::from(&extended)
    }
}

/// This is a pre-processed version of an affine point `(x, y)`
/// in the form `(y + x, y - x, x * y * 2d)`. This can be added to an
/// [`JubJubExtended`](crate::JubJubExtended).
#[derive(Clone, Copy, Debug)]
pub struct AffineNielsPoint {
    y_plus_x: Fr,
    y_minus_x: Fr,
    t2d: Fr,
}

impl AffineNielsPoint {
    /// Constructs this point from the neutral element `(0, 1)`.
    pub const fn identity() -> Self {
        AffineNielsPoint {
            y_plus_x: Fr::one(),
            y_minus_x: Fr::one(),
            t2d: Fr::zero(),
        }
    }

    #[inline]
    fn multiply(&self, by: &[u8; 32]) -> JubJubExtended {
        let zero = AffineNielsPoint::identity();

        let mut acc = JubJubExtended::identity();

        // This is a simple double-and-add implementation of point
        // multiplication, moving from most significant to least
        // significant bit of the scalar.
        //
        // We skip the leading four bits because they're always
        // unset for Fr.
        for bit in by
            .iter()
            .rev()
            .flat_map(|byte| (0..8).rev().map(move |i| Choice::from((byte >> i) & 1u8)))
            .skip(4)
        {
            acc = acc.double();
            acc += AffineNielsPoint::conditional_select(&zero, &self, bit);
        }

        acc
    }

    /// Multiplies this point by the specific little-endian bit pattern in the
    /// given byte array, ignoring the highest four bits.
    pub fn multiply_bits(&self, by: &[u8; 32]) -> JubJubExtended {
        self.multiply(by)
    }
}

impl<'a, 'b> Mul<&'b Fr> for &'a AffineNielsPoint {
    type Output = JubJubExtended;

    fn mul(self, other: &'b Fr) -> JubJubExtended {
        self.multiply(&other.to_bytes())
    }
}

impl_binops_multiplicative_mixed!(AffineNielsPoint, Fr, JubJubExtended);

impl ConditionallySelectable for AffineNielsPoint {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        AffineNielsPoint {
            y_plus_x: Fr::conditional_select(&a.y_plus_x, &b.y_plus_x, choice),
            y_minus_x: Fr::conditional_select(&a.y_minus_x, &b.y_minus_x, choice),
            t2d: Fr::conditional_select(&a.t2d, &b.t2d, choice),
        }
    }
}

/// This is a pre-processed version of an extended point `(X, Y, Z, T1, T2)`
/// in the form `(Y + X, Y - X, Z, T1 * T2 * 2d)`.
#[derive(Clone, Copy, Debug)]
pub struct ExtendedNielsPoint {
    y_plus_x: Fr,
    y_minus_x: Fr,
    z: Fr,
    t2d: Fr,
}

impl ConditionallySelectable for ExtendedNielsPoint {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        ExtendedNielsPoint {
            y_plus_x: Fr::conditional_select(&a.y_plus_x, &b.y_plus_x, choice),
            y_minus_x: Fr::conditional_select(&a.y_minus_x, &b.y_minus_x, choice),
            z: Fr::conditional_select(&a.z, &b.z, choice),
            t2d: Fr::conditional_select(&a.t2d, &b.t2d, choice),
        }
    }
}

impl ExtendedNielsPoint {
    /// Constructs this point from the neutral element `(0, 1)`.
    pub const fn identity() -> Self {
        ExtendedNielsPoint {
            y_plus_x: Fr::one(),
            y_minus_x: Fr::one(),
            z: Fr::one(),
            t2d: Fr::zero(),
        }
    }

    #[inline]
    fn multiply(&self, by: &[u8; 32]) -> JubJubExtended {
        let zero = ExtendedNielsPoint::identity();

        let mut acc = JubJubExtended::identity();

        // This is a simple double-and-add implementation of point
        // multiplication, moving from most significant to least
        // significant bit of the scalar.
        //
        // We skip the leading four bits because they're always
        // unset for Fr.
        for bit in by
            .iter()
            .rev()
            .flat_map(|byte| (0..8).rev().map(move |i| Choice::from((byte >> i) & 1u8)))
            .skip(4)
        {
            acc = acc.double();
            acc += ExtendedNielsPoint::conditional_select(&zero, &self, bit);
        }

        acc
    }

    /// Multiplies this point by the specific little-endian bit pattern in the
    /// given byte array, ignoring the highest four bits.
    pub fn multiply_bits(&self, by: &[u8; 32]) -> JubJubExtended {
        self.multiply(by)
    }
}

impl<'a, 'b> Mul<&'b Fr> for &'a ExtendedNielsPoint {
    type Output = JubJubExtended;

    fn mul(self, other: &'b Fr) -> JubJubExtended {
        self.multiply(&other.to_bytes())
    }
}

impl_binops_multiplicative_mixed!(ExtendedNielsPoint, Fr, JubJubExtended);

/// `d = -(10240/10241)`
pub const EDWARDS_D: Fr = Fr::to_mont_form([
    0x01065fd6d6343eb1,
    0x292d7f6d37579d26,
    0xf5fd9207e6bd7fd4,
    0x2a9318e74bfa2b48,
]);

/// `2*EDWARDS_D`
pub const EDWARDS_D2: Fr = Fr::to_mont_form([
    0x020cbfadac687d62,
    0x525afeda6eaf3a4c,
    0xebfb240fcd7affa8,
    0x552631ce97f45691,
]);

impl Serializable<32> for JubJubAffine {
    type Error = BytesError;

    /// Converts this element into its byte representation.
    fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut tmp = self.y.to_bytes();
        let x = self.x.to_bytes();

        // Encode the sign of the x-coordinate in the most
        // significant bit.
        tmp[31] |= x[0] << 7;

        tmp
    }

    /// Attempts to interpret a byte representation of an
    /// affine point, failing if the element is not on
    /// the curve or non-canonical.
    ///
    /// NOTE: ZIP 216 is enabled by default and the only way to interact with
    /// serialization.
    /// See: <https://zips.z.cash/zip-0216> for more details.
    fn from_bytes(b: &[u8; Self::SIZE]) -> Result<Self, Self::Error> {
        let mut b = b.clone();

        // Grab the sign bit from the representation
        let sign = b[31] >> 7;

        // Mask away the sign bit
        b[31] &= 0b0111_1111;

        // Interpret what remains as the y-coordinate
        let y = Fr::from_bytes(&b)?;

        // -x^2 + y^2 = 1 + d.x^2.y^2
        // -x^2 = 1 + d.x^2.y^2 - y^2    (rearrange)
        // -x^2 - d.x^2.y^2 = 1 - y^2    (rearrange)
        // x^2 + d.x^2.y^2 = y^2 - 1     (flip signs)
        // x^2 (1 + d.y^2) = y^2 - 1     (factor)
        // x^2 = (y^2 - 1) / (1 + d.y^2) (isolate x^2)
        // We know that (1 + d.y^2) is nonzero for all y:
        //   (1 + d.y^2) = 0
        //   d.y^2 = -1
        //   y^2 = -(1 / d)   No solutions, as -(1 / d) is not a square

        let y2 = y.square();

        Option::from(
            ((y2 - Fr::one()) * ((Fr::one() + EDWARDS_D * &y2).invert().unwrap_or(Fr::zero())))
                .sqrt()
                .and_then(|x| {
                    // Fix the sign of `x` if necessary
                    let flip_sign = Choice::from((x.to_bytes()[0] ^ sign) & 1);
                    let x = Fr::conditional_select(&x, &-x, flip_sign);
                    // If x == 0, flip_sign == sign_bit. We therefore want to reject
                    // the encoding as non-canonical if all of the
                    // following occur:
                    // - x == 0
                    // - flip_sign == true
                    let x_is_zero = x.ct_eq(&Fr::zero());
                    CtOption::new(JubJubAffine { x, y }, !(x_is_zero & flip_sign))
                }),
        )
        .ok_or(BytesError::InvalidData)
    }
}

impl JubJubAffine {
    /// Constructs the neutral element `(0, 1)`.
    pub const fn identity() -> Self {
        JubJubAffine {
            x: Fr::zero(),
            y: Fr::one(),
        }
    }

    /// Constructs an JubJubAffine given `x` and `y` without checking
    /// that the point is on the curve.
    pub const fn from_raw_unchecked(x: Fr, y: Fr) -> JubJubAffine {
        JubJubAffine { x, y }
    }

    /// Multiplies this point by the cofactor, producing an
    /// `JubJubExtended`
    pub fn mul_by_cofactor(&self) -> JubJubExtended {
        JubJubExtended::from(*self).mul_by_cofactor()
    }

    /// Determines if this point is of small order.
    pub fn is_small_order(&self) -> Choice {
        JubJubExtended::from(*self).is_small_order()
    }

    /// Determines if this point is torsion free and so is
    /// in the prime order subgroup.
    pub fn is_torsion_free(&self) -> Choice {
        JubJubExtended::from(*self).is_torsion_free()
    }

    /// Determines if this point is prime order, or in other words that
    /// the smallest scalar multiplied by this point that produces the
    /// identity is `r`. This is equivalent to checking that the point
    /// is both torsion free and not the identity.
    pub fn is_prime_order(&self) -> Choice {
        let extended = JubJubExtended::from(*self);
        extended.is_torsion_free() & (!extended.is_identity())
    }

    /// Returns the `x`-coordinate of this point.
    pub const fn get_x(&self) -> Fr {
        self.x
    }

    /// Returns the `y`-coordinate of this point.
    pub const fn get_y(&self) -> Fr {
        self.y
    }

    /// Performs a pre-processing step that produces an `AffineNielsPoint`
    /// for use in multiple additions.
    pub fn to_niels(&self) -> AffineNielsPoint {
        AffineNielsPoint {
            y_plus_x: Fr::add(self.y, &self.x),
            y_minus_x: Fr::sub(self.y, &self.x),
            t2d: Fr::mul(Fr::mul(self.x, &self.y), &EDWARDS_D2),
        }
    }

    /// Constructs an JubJubAffine given `x` and `y` without checking
    /// that the point is on the curve.
    pub const fn to_mont_form_unchecked(x: Fr, y: Fr) -> JubJubAffine {
        JubJubAffine { x, y }
    }

    /// This is only for debugging purposes and not
    /// exposed in the public API. Checks that this
    /// point is on the curve.
    #[cfg(test)]
    fn is_on_curve_vartime(&self) -> bool {
        let x2 = self.x.square();
        let y2 = self.y.square();

        &y2 - &x2 == Fr::one() + &EDWARDS_D * &x2 * &y2
    }
}

impl JubJubExtended {
    /// Constructs an extended point (with `Z = 1`) from
    /// an affine point using the map `(x, y) => (x, y, 1, x, y)`.
    pub const fn from_affine(affine: JubJubAffine) -> Self {
        Self::to_mont_form_unchecked(affine.x, affine.y, Fr::one(), affine.x, affine.y)
    }

    /// Constructs an extended point from its raw internals
    pub const fn to_mont_form_unchecked(x: Fr, y: Fr, z: Fr, t1: Fr, t2: Fr) -> Self {
        JubJubExtended { x, y, z, t1, t2 }
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

    /// Returns the `t1`-coordinate of this point.
    pub const fn get_t1(&self) -> Fr {
        self.t1
    }

    /// Returns the `t2`-coordinate of this point.
    pub const fn get_t2(&self) -> Fr {
        self.t2
    }

    /// Constructs an extended point from the neutral element `(0, 1)`.
    pub const fn identity() -> Self {
        JubJubExtended {
            x: Fr::zero(),
            y: Fr::one(),
            z: Fr::one(),
            t1: Fr::zero(),
            t2: Fr::zero(),
        }
    }

    /// Determines if this point is the identity.
    pub fn is_identity(&self) -> Choice {
        // If this point is the identity, then
        //     x = 0 * z = 0
        // and y = 1 * z = z
        self.x.ct_eq(&Fr::zero()) & self.y.ct_eq(&self.z)
    }

    /// Determines if this point is of small order.
    pub fn is_small_order(&self) -> Choice {
        // We only need to perform two doublings, since the 2-torsion
        // points are (0, 1) and (0, -1), and so we only need to check
        // that the x-coordinate of the result is zero to see if the
        // point is small order.
        self.double().double().x.ct_eq(&Fr::zero())
    }

    /// Determines if this point is torsion free and so is contained
    /// in the prime order subgroup.
    pub fn is_torsion_free(&self) -> Choice {
        self.multiply(&FR_MODULUS_BYTES).is_identity()
    }

    /// Determines if this point is prime order, or in other words that
    /// the smallest scalar multiplied by this point that produces the
    /// identity is `r`. This is equivalent to checking that the point
    /// is both torsion free and not the identity.
    pub fn is_prime_order(&self) -> Choice {
        self.is_torsion_free() & (!self.is_identity())
    }

    /// Multiplies this element by the cofactor `8`.
    pub fn mul_by_cofactor(&self) -> JubJubExtended {
        self.double().double().double()
    }

    /// Performs a pre-processing step that produces an `ExtendedNielsPoint`
    /// for use in multiple additions.
    pub fn to_niels(&self) -> ExtendedNielsPoint {
        ExtendedNielsPoint {
            y_plus_x: &self.y + &self.x,
            y_minus_x: &self.y - &self.x,
            z: self.z,
            t2d: &self.t1 * &self.t2 * EDWARDS_D2,
        }
    }

    /// Returns two scalars suitable for hashing that represent the
    /// Extended Point.
    pub fn to_hash_inputs(&self) -> [Fr; 2] {
        // The same JubJubAffine can have different JubJubExtended
        // representations, therefore we convert from Extended to Affine
        // before hashing, to ensure deterministic result
        let p = JubJubAffine::from(self);
        [p.x, p.y]
    }

    /// Computes the doubling of a point more efficiently than a point can
    /// be added to itself.
    pub fn double(&self) -> JubJubExtended {
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
        let xy2 = (&self.x + &self.y).square();
        let yy_plus_xx = &yy + &xx;
        let yy_minus_xx = &yy - &xx;

        // The remaining arithmetic is exactly the process of converting
        // from a completed point to an extended point.
        CompletedPoint {
            x: &xy2 - &yy_plus_xx,
            y: yy_plus_xx,
            z: yy_minus_xx,
            t: &zz2 - &yy_minus_xx,
        }
        .into_extended()
    }

    #[inline]
    fn multiply(self, by: &[u8; 32]) -> Self {
        self.to_niels().multiply(by)
    }

    /// This is only for debugging purposes and not
    /// exposed in the public API. Checks that this
    /// point is on the curve.
    #[cfg(test)]
    fn is_on_curve_vartime(&self) -> bool {
        let affine = JubJubAffine::from(*self);

        self.z != Fr::zero()
            && affine.is_on_curve_vartime()
            && (affine.x * affine.y * self.z == self.t1 * self.t2)
    }
}

impl<'a, 'b> Mul<&'b Fr> for &'a JubJubExtended {
    type Output = JubJubExtended;

    fn mul(self, other: &'b Fr) -> JubJubExtended {
        self.multiply(&other.to_bytes())
    }
}

impl_binops_multiplicative!(JubJubExtended, Fr);

impl<'a, 'b> Add<&'b ExtendedNielsPoint> for &'a JubJubExtended {
    type Output = JubJubExtended;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add(self, other: &'b ExtendedNielsPoint) -> JubJubExtended {
        // We perform addition in the extended coordinates. Here we use
        // a formula presented by Hisil, Wong, Carter and Dawson in
        // "Twisted Edward Curves Revisited" which only requires 8M.
        //
        // A = (Y1 - X1) * (Y2 - X2)
        // B = (Y1 + X1) * (Y2 + X2)
        // C = 2d * T1 * T2
        // D = 2 * Z1 * Z2
        // E = B - A
        // F = D - C
        // G = D + C
        // H = B + A
        // X3 = E * F
        // Y3 = G * H
        // Z3 = F * G
        // T3 = E * H

        let a = (&self.y - &self.x) * &other.y_minus_x;
        let b = (&self.y + &self.x) * &other.y_plus_x;
        let c = &self.t1 * &self.t2 * &other.t2d;
        let d = (&self.z * &other.z).double();

        // The remaining arithmetic is exactly the process of converting
        // from a completed point to an extended point.
        CompletedPoint {
            x: &b - &a,
            y: &b + &a,
            z: &d + &c,
            t: &d - &c,
        }
        .into_extended()
    }
}

impl<'a, 'b> Sub<&'b ExtendedNielsPoint> for &'a JubJubExtended {
    type Output = JubJubExtended;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn sub(self, other: &'b ExtendedNielsPoint) -> JubJubExtended {
        let a = (&self.y - &self.x) * &other.y_plus_x;
        let b = (&self.y + &self.x) * &other.y_minus_x;
        let c = &self.t1 * &self.t2 * &other.t2d;
        let d = (&self.z * &other.z).double();

        CompletedPoint {
            x: &b - &a,
            y: &b + &a,
            z: &d - &c,
            t: &d + &c,
        }
        .into_extended()
    }
}

impl_binops_additive!(JubJubExtended, ExtendedNielsPoint);

impl<'a, 'b> Add<&'b AffineNielsPoint> for &'a JubJubExtended {
    type Output = JubJubExtended;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add(self, other: &'b AffineNielsPoint) -> JubJubExtended {
        // This is identical to the addition formula for `ExtendedNielsPoint`,
        // except we can assume that `other.z` is one, so that we perform
        // 7 multiplications.

        let a = (&self.y - &self.x) * &other.y_minus_x;
        let b = (&self.y + &self.x) * &other.y_plus_x;
        let c = &self.t1 * &self.t2 * &other.t2d;
        let d = self.z.double();

        // The remaining arithmetic is exactly the process of converting
        // from a completed point to an extended point.
        CompletedPoint {
            x: &b - &a,
            y: &b + &a,
            z: &d + &c,
            t: &d - &c,
        }
        .into_extended()
    }
}

impl<'a, 'b> Sub<&'b AffineNielsPoint> for &'a JubJubExtended {
    type Output = JubJubExtended;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn sub(self, other: &'b AffineNielsPoint) -> JubJubExtended {
        let a = (&self.y - &self.x) * &other.y_plus_x;
        let b = (&self.y + &self.x) * &other.y_minus_x;
        let c = &self.t1 * &self.t2 * &other.t2d;
        let d = self.z.double();

        CompletedPoint {
            x: &b - &a,
            y: &b + &a,
            z: &d - &c,
            t: &d + &c,
        }
        .into_extended()
    }
}

impl_binops_additive!(JubJubExtended, AffineNielsPoint);

impl<'a, 'b> Add<&'b JubJubExtended> for &'a JubJubExtended {
    type Output = JubJubExtended;

    #[inline]
    fn add(self, other: &'b JubJubExtended) -> JubJubExtended {
        self + other.to_niels()
    }
}

impl<'a, 'b> Sub<&'b JubJubExtended> for &'a JubJubExtended {
    type Output = JubJubExtended;

    #[inline]
    fn sub(self, other: &'b JubJubExtended) -> JubJubExtended {
        self - other.to_niels()
    }
}

impl_binops_additive!(JubJubExtended, JubJubExtended);

impl<'a, 'b> Add<&'b JubJubAffine> for &'a JubJubExtended {
    type Output = JubJubExtended;

    #[inline]
    fn add(self, other: &'b JubJubAffine) -> JubJubExtended {
        self + other.to_niels()
    }
}

impl<'a, 'b> Sub<&'b JubJubAffine> for &'a JubJubExtended {
    type Output = JubJubExtended;

    #[inline]
    fn sub(self, other: &'b JubJubAffine) -> JubJubExtended {
        self - other.to_niels()
    }
}

impl_binops_additive!(JubJubExtended, JubJubAffine);

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
/// returned which can be used to obtain `JubJubAffine`s for each element in the
/// slice.
///
/// This costs 5 multiplications per element, and a field inversion.
pub fn batch_normalize<'a>(y: &'a mut [JubJubExtended]) -> impl Iterator<Item = JubJubAffine> + 'a {
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

    y.iter().map(|p| JubJubAffine { x: p.x, y: p.y })
}

impl<'a, 'b> Mul<&'b Fp> for &'a AffineNielsPoint {
    type Output = JubJubExtended;

    fn mul(self, other: &'b Fp) -> JubJubExtended {
        self.multiply(&other.to_bytes())
    }
}

impl_binops_multiplicative_mixed!(AffineNielsPoint, Fp, JubJubExtended);

impl<'a, 'b> Mul<&'b Fp> for &'a JubJubExtended {
    type Output = JubJubExtended;

    fn mul(self, other: &'b Fp) -> JubJubExtended {
        self.multiply(&other.to_bytes())
    }
}

impl_binops_multiplicative!(JubJubExtended, Fp);

impl Eq for JubJubAffine {}

impl Default for JubJubAffine {
    fn default() -> Self {
        JubJubAffine::identity()
    }
}
