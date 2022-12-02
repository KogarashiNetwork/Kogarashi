// Copyright (C) 2020-2023 Invers (JP) INC.
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

//! # Bls12 381 Curve Implementation
//!
//! - [`Fq`]
//! - [`Fr`]
//! - [`G1Projective`]
//! - [`G2Projective`]
//!
//! ## Overview
//!
//! This curve provides the functionalities as following.
//!
//! - Point addition
//! - Point doubling
//! - Point scalar
//! - G1 and G2 group operation
//!
//! ### Reference
//!
//! We implement coordinate system to refer the following.
//! [Projective coordinates for short Weierstrass curves](https://www.hyperelliptic.org/EFD/g1p/auto-shortw-projective.html)

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)]

mod fq;
mod fq2;
mod fq6;
mod fr;
mod g1;
mod g2;

pub use fq::Fq;
pub use fr::Fr;
pub use g1::{G1Affine, G1Projective};
pub use g2::{G2Affine, G2Projective};
