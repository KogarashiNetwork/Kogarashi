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

mod keccak256;
mod utils;

pub use keccak256::keccak256;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // https://emn178.github.io/online-tools/sha3_256.html
        let msg = b"hello";
        let result = b"3338be694f50c5f338814986cdf0686453a888b84f424d792af4b9202398f392".to_vec();
        let digest = keccak256(msg.to_vec());

        // assert_eq!(digest, result);
    }
}
