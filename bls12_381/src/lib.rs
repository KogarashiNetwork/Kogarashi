// Copyright (C) 2023-2024 Invers (JP) INC.
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
#![allow(clippy::suspicious_arithmetic_impl)]
#![allow(clippy::suspicious_op_assign_impl)]
#![allow(dead_code)]

mod error;
mod fq;
mod fqn;
mod fr;
mod g1;
mod g2;
mod gt;
pub mod params;

pub use fq::Fq;
pub use fqn::{Fq12, Fq2, Fq6};
pub use fr::{Fr, MULTIPLICATIVE_GENERATOR, ROOT_OF_UNITY, TWO_ADACITY};
pub use g1::{G1Affine, G1Projective};
pub use g2::{G2Affine, G2PairingAffine, G2Projective, PairingCoeff};
pub use gt::Gt;
pub use params::EDWARDS_D;
