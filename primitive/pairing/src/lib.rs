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
use zkstd::common::{G2Pairing, Pairing, PairingRange, PrimeField, Vec};

/// Tate pairing struct holds necessary components for pairing.
/// `pairing` function takes G1 and G2 group elements and output
/// GT target group element.
#[derive(Debug, Clone, Eq, PartialEq, Default, Encode, Decode, Copy)]
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
    bases: &[P::G1Affine],
    coeffs: &[P::ScalarField],
) -> P::G1Projective {
    let c = if bases.len() < 4 {
        1
    } else if bases.len() < 32 {
        3
    } else {
        let log2 = usize::BITS - bases.len().leading_zeros();
        (log2 * 69 / 100) as usize + 2
    };
    let mut buckets: Vec<Vec<Bucket<P>>> = vec![vec![Bucket::None; (1 << c) - 1]; (256 / c) + 1];

    let new_buckets = buckets
        .iter_mut()
        .enumerate()
        .rev()
        .map(|(i, bucket)| {
            for (coeff, base) in coeffs.iter().zip(bases.iter()) {
                let seg = get_at(i, c, coeff.to_bytes());
                if seg != 0 {
                    bucket[seg - 1].add_assign(base);
                }
            }
            // Summation by parts
            // e.g. 3a + 2b + 1c = a +
            //                    (a) + b +
            //                    ((a) + b) + c
            let mut acc = P::G1Projective::ADDITIVE_IDENTITY;
            let mut sum = P::G1Projective::ADDITIVE_IDENTITY;
            bucket.iter().rev().for_each(|b| {
                sum = b.add(sum);
                acc += sum;
            });
            acc
        })
        .collect::<Vec<_>>();
    new_buckets
        .iter()
        .fold(P::G1Projective::ADDITIVE_IDENTITY, |mut sum, bucket| {
            (0..c).for_each(|_| sum = sum.double());
            sum + bucket
        })
}

#[derive(Clone, Copy)]
enum Bucket<P: Pairing> {
    None,
    Affine(P::G1Affine),
    Projective(P::G1Projective),
}

impl<P: Pairing> Bucket<P> {
    fn add_assign(&mut self, other: &P::G1Affine) {
        *self = match *self {
            Bucket::None => Bucket::Affine(*other),
            Bucket::Affine(a) => Bucket::Projective(a + other),
            Bucket::Projective(a) => Bucket::Projective(a + other),
        }
    }

    fn add(&self, other: P::G1Projective) -> P::G1Projective {
        match self {
            Bucket::None => other,
            Bucket::Affine(a) => other + a,
            Bucket::Projective(a) => other + a,
        }
    }
}

fn get_at(segment: usize, c: usize, bytes: [u8; 32]) -> usize {
    let skip_bits = segment * c;
    let skip_bytes = skip_bits / 8;

    if skip_bytes >= 32 {
        0
    } else {
        let mut v = [0; 8];
        for (v, o) in v.iter_mut().zip(bytes[skip_bytes..].iter()) {
            *v = *o;
        }

        let mut tmp = u64::from_le_bytes(v);
        tmp >>= skip_bits - (skip_bytes * 8);
        (tmp % (1 << c)) as usize
    }
}
