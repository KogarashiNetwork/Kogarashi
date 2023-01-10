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

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
mod commitment;
#[cfg(feature = "std")]
mod fft;
#[cfg(feature = "std")]
mod keypair;
#[cfg(feature = "std")]
mod poly;
#[cfg(feature = "std")]
mod witness;

#[cfg(feature = "std")]
pub use commitment::KzgCommitment;
#[cfg(feature = "std")]
pub use fft::Fft;
#[cfg(feature = "std")]
pub use keypair::KeyPair;
#[cfg(feature = "std")]
pub use witness::Witness;
