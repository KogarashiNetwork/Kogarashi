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

#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

mod commitment;
mod fft;
mod keypair;
mod poly;
mod util;
mod witness;

pub use commitment::Commitment;
pub use fft::Fft;
pub use keypair::Error as KzgError;
pub use keypair::KeyPair;
pub use poly::{Coefficients, PointsValue};
pub use util::{batch_inversion, powers_of};
pub use witness::Witness;
