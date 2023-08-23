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

#![no_std]
#![doc = include_str!("../README.md")]

use bls_12_381::params::{BLS_X, BLS_X_IS_NEGATIVE};
use bls_12_381::{Fq12, Fr, G1Affine, G1Projective, G2Affine, G2PairingAffine, G2Projective, Gt};
use jub_jub::{Fp, JubjubAffine, JubjubExtended};
use zkstd::common::*;
use zkstd::common::{G2Pairing, Group, Pairing, PairingRange, PrimeField, Ring, Vec};

/// Tate pairing struct holds necessary components for pairing.
/// `pairing` function takes G1 and G2 group elements and output
/// GT target group element.
#[derive(Debug, Clone, Eq, PartialEq, Default, Encode, Decode)]
pub struct TatePairing;

impl Pairing for TatePairing {
    type G1Affine = G1Affine;
    type G2Affine = G2Affine;
    type G1Projective = G1Projective;
    type G2Projective = G2Projective;
    type JubjubAffine = JubjubAffine;
    type JubjubExtended = JubjubExtended;
    type G2PairngRepr = G2PairingAffine;
    type PairingRange = Fq12;
    type Gt = Gt;
    type ScalarField = Fr;
    type JubjubScalar = Fp;
    const X: u64 = BLS_X;
    const X_IS_NEGATIVE: bool = BLS_X_IS_NEGATIVE;

    fn pairing(g1: Self::G1Affine, g2: Self::G2Affine) -> Self::Gt {
        Self::miller_loop(g1, g2).final_exp()
    }

    fn miller_loop(g1: Self::G1Affine, g2: Self::G2Affine) -> Self::PairingRange {
        let mut acc = Self::PairingRange::one();
        let mut g2_projective = Self::G2Projective::from(g2);
        let mut found_one = false;

        for i in (0..64).rev().map(|b| (((BLS_X >> 1) >> b) & 1) == 1) {
            if !found_one {
                found_one = i;
                continue;
            }

            acc = acc.untwist(g2_projective.double_eval(), g1);

            if i {
                acc = acc.untwist(g2_projective.add_eval(g2), g1);
            }

            acc.square_assign();
        }

        acc = acc.untwist(g2_projective.double_eval(), g1);

        if Self::X_IS_NEGATIVE {
            acc.conjugate()
        } else {
            acc
        }
    }

    fn multi_miller_loop(pairs: &[(Self::G1Affine, Self::G2PairngRepr)]) -> Self::PairingRange {
        let pairs = pairs
            .iter()
            .filter(|(a, b)| !a.is_identity() && !b.is_identity())
            .collect::<Vec<_>>();
        let mut acc = Self::PairingRange::one();
        let mut counter = 0;
        let mut found_one = false;

        for i in (0..64).rev().map(|b| (((BLS_X >> 1) >> b) & 1) == 1) {
            if !found_one {
                found_one = i;
                continue;
            }

            for (g1, g2) in pairs.iter() {
                acc = acc.untwist(g2.coeffs[counter], *g1);
            }
            counter += 1;

            if i {
                for (g1, g2) in pairs.iter() {
                    acc = acc.untwist(g2.coeffs[counter], *g1);
                }
                counter += 1;
            }

            acc.square_assign();
        }

        for (g1, g2) in pairs {
            acc = acc.untwist(g2.coeffs[counter], *g1);
        }

        if Self::X_IS_NEGATIVE {
            acc.conjugate()
        } else {
            acc
        }
    }
}

/// Performs a Variable Base Multiscalar Multiplication.
pub fn msm_curve_addtion<P: Pairing>(
    points: &[P::G1Affine],
    scalars: &[P::ScalarField],
) -> P::G1Projective {
    let c = if scalars.len() < 32 {
        3
    } else {
        ln_without_floats(scalars.len()) + 2
    };

    let num_bits = 255usize;
    let fr_one = P::ScalarField::one();

    let zero = P::G1Projective::ADDITIVE_IDENTITY;

    let window_starts_iter = (0..num_bits).step_by(c);

    // Each window is of size `c`.
    // We divide up the bits 0..num_bits into windows of size `c`, and
    // in parallel process each such window.
    let window_sums: Vec<_> = window_starts_iter
        .map(|w_start| {
            let mut res = zero;
            // We don't need the "zero" bucket, so we only have 2^c - 1 buckets
            let mut buckets = vec![zero; (1 << c) - 1];
            scalars
                .iter()
                .zip(points)
                .filter(|(s, _)| *s != &P::ScalarField::zero())
                .for_each(|(&scalar, base)| {
                    if scalar == fr_one {
                        // We only process unit scalars once in the first window.
                        if w_start == 0 {
                            res += *base;
                        }
                    } else {
                        let mut scalar = scalar.reduce();

                        // We right-shift by w_start, thus getting rid of the
                        // lower bits.
                        scalar.divn(w_start as u32);

                        // We mod the remaining bits by the window size.
                        let scalar = scalar.mod_by_window(c);

                        // If the scalar is non-zero, we update the corresponding
                        // bucket.
                        // (Recall that `buckets` doesn't have a zero bucket.)
                        if scalar != 0 {
                            buckets[(scalar - 1) as usize] += *base;
                        }
                    }
                });

            let mut running_sum = P::G1Projective::ADDITIVE_IDENTITY;
            for b in buckets.into_iter().rev() {
                running_sum += b;
                res += running_sum;
            }

            res
        })
        .collect();

    // We store the sum for the lowest window.
    let lowest = *window_sums.first().unwrap();
    // We're traversing windows from high to low.
    let x = window_sums[1..]
        .iter()
        .rev()
        .fold(zero, |mut total, sum_i| {
            total += *sum_i;
            for _ in 0..c {
                total = total.double();
            }
            total
        })
        + lowest;

    x
}

fn ln_without_floats(a: usize) -> usize {
    // log2(a) * ln(2)
    (log2(a) * 69 / 100) as usize
}

fn log2(x: usize) -> u32 {
    if x <= 1 {
        return 0;
    }

    let n = x.leading_zeros();
    core::mem::size_of::<usize>() as u32 * 8 - n
}
