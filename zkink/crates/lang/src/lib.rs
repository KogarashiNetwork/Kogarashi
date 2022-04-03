// Copyright 2018-2022 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
#[doc(hidden)]
pub mod result_info;

#[cfg_attr(not(feature = "show-codegen-docs"), doc(hidden))]
pub mod codegen;

/// Utility functions for contract development.
pub mod utils {
    // We want to expose this function without making users go through
    // the `codgen` module
    pub use super::codegen::initialize_contract;
}

pub mod reflect;

mod chain_extension;
mod contract_ref;
mod env_access;

pub use self::{
    chain_extension::{
        ChainExtensionInstance,
        IsResultType,
    },
    contract_ref::ToAccountId,
    env_access::EnvAccess,
};
pub use ink_lang_macro::{
    blake2x256,
    chain_extension,
    contract,
    selector_bytes,
    selector_id,
    test,
    trait_definition,
};
