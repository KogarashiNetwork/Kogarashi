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

//! Types for the default environment.
//!
//! These are simple mirrored types from the default substrate FRAME configuration.
//! Their interfaces and functionality might not be complete.
//!
//! Users are required to provide their own type definitions and `Environment`
//! implementations in order to write ink! contracts for other chain configurations.
//!
//! # Note
//!
//! When authoring a contract, the concrete `Environment` are available via aliases
//! generated by the `lang` macro. Therefore all functionality of the concrete
//! types is accessible in the contract, not constrained by the required trait
//! bounds.
//!
//! Outside the contract and its tests (e.g. in the off-chain environment), where
//! there is no knowledge of the concrete types, the functionality is restricted to
//! the trait bounds on the `Environment` trait types.

use super::arithmetic::AtLeast32BitUnsigned;
use core::array::TryFromSliceError;
use derive_more::From;
use scale::{Decode, Encode};
#[cfg(feature = "std")]
use scale_info::TypeInfo;

/// Allows to instantiate a type from its little-endian bytes representation.
pub trait FromLittleEndian {
    /// The little-endian bytes representation.
    type Bytes: Default + AsRef<[u8]> + AsMut<[u8]>;

    /// Create a new instance from the little-endian bytes representation.
    fn from_le_bytes(bytes: Self::Bytes) -> Self;
}

impl FromLittleEndian for u8 {
    type Bytes = [u8; 1];

    #[inline]
    fn from_le_bytes(bytes: Self::Bytes) -> Self {
        u8::from_le_bytes(bytes)
    }
}

impl FromLittleEndian for u16 {
    type Bytes = [u8; 2];

    #[inline]
    fn from_le_bytes(bytes: Self::Bytes) -> Self {
        u16::from_le_bytes(bytes)
    }
}

impl FromLittleEndian for u32 {
    type Bytes = [u8; 4];

    #[inline]
    fn from_le_bytes(bytes: Self::Bytes) -> Self {
        u32::from_le_bytes(bytes)
    }
}

impl FromLittleEndian for u64 {
    type Bytes = [u8; 8];

    #[inline]
    fn from_le_bytes(bytes: Self::Bytes) -> Self {
        u64::from_le_bytes(bytes)
    }
}

impl FromLittleEndian for u128 {
    type Bytes = [u8; 16];

    #[inline]
    fn from_le_bytes(bytes: Self::Bytes) -> Self {
        u128::from_le_bytes(bytes)
    }
}

/// The environmental types usable by contracts defined with ink!.
pub trait Environment {
    /// The maximum number of supported event topics provided by the runtime.
    ///
    /// The value must match the maximum number of supported event topics of the used runtime.
    const MAX_EVENT_TOPICS: usize;

    /// The address type.
    type AccountId: 'static
        + scale::Codec
        + Clone
        + PartialEq
        + Eq
        + Ord
        + AsRef<[u8]>
        + AsMut<[u8]>
        + Default;

    /// The type of balances.
    type Balance: 'static
        + scale::Codec
        + Copy
        + Clone
        + PartialEq
        + Eq
        + AtLeast32BitUnsigned
        + FromLittleEndian;

    /// The type of hash.
    type Hash: 'static
        + scale::Codec
        + Copy
        + Clone
        + Clear
        + PartialEq
        + Eq
        + Ord
        + AsRef<[u8]>
        + AsMut<[u8]>;

    /// The type of a timestamp.
    type Timestamp: 'static
        + scale::Codec
        + Copy
        + Clone
        + PartialEq
        + Eq
        + AtLeast32BitUnsigned
        + FromLittleEndian;

    /// The type of block number.
    type BlockNumber: 'static
        + scale::Codec
        + Copy
        + Clone
        + PartialEq
        + Eq
        + AtLeast32BitUnsigned
        + FromLittleEndian;

    /// The chain extension for the environment.
    ///
    /// This is a type that is defined through the `#[ink::chain_extension]` procedural macro.
    /// For more information about usage and definition click [this][chain_extension] link.
    ///
    /// [chain_extension]: https://paritytech.github.io/ink/ink_lang/attr.chain_extension.html
    type ChainExtension;
}

/// Placeholder for chains that have no defined chain extension.
pub enum NoChainExtension {}

/// The fundamental types of the default configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(TypeInfo))]
pub enum DefaultEnvironment {}

impl Environment for DefaultEnvironment {
    const MAX_EVENT_TOPICS: usize = 4;

    type AccountId = AccountId;
    type Balance = Balance;
    type Hash = Hash;
    type Timestamp = Timestamp;
    type BlockNumber = BlockNumber;
    type ChainExtension = NoChainExtension;
}

/// The default balance type.
pub type Balance = u128;

/// The default timestamp type.
pub type Timestamp = u64;

/// The default gas type.
pub type Gas = u64;

/// The default block number type.
pub type BlockNumber = u32;

/// The default environment `AccountId` type.
///
/// # Note
///
/// This is a mirror of the `AccountId` type used in the default configuration
/// of PALLET contracts.
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Encode, Decode, From, Default,
)]
#[cfg_attr(feature = "std", derive(TypeInfo))]
pub struct AccountId([u8; 32]);

impl AsRef<[u8; 32]> for AccountId {
    #[inline]
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl AsMut<[u8; 32]> for AccountId {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8; 32] {
        &mut self.0
    }
}

impl AsRef<[u8]> for AccountId {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl AsMut<[u8]> for AccountId {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0[..]
    }
}

impl<'a> TryFrom<&'a [u8]> for AccountId {
    type Error = TryFromSliceError;

    fn try_from(bytes: &'a [u8]) -> Result<Self, TryFromSliceError> {
        let address = <[u8; 32]>::try_from(bytes)?;
        Ok(Self(address))
    }
}

/// The default environment `Hash` type.
///
/// # Note
///
/// This is a mirror of the `Hash` type used in the default configuration
/// of PALLET contracts.
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Encode, Decode, From, Default,
)]
#[cfg_attr(feature = "std", derive(TypeInfo))]
pub struct Hash([u8; 32]);

impl<'a> TryFrom<&'a [u8]> for Hash {
    type Error = TryFromSliceError;

    fn try_from(bytes: &'a [u8]) -> Result<Self, TryFromSliceError> {
        let address = <[u8; 32]>::try_from(bytes)?;
        Ok(Self(address))
    }
}

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl AsMut<[u8]> for Hash {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0[..]
    }
}

/// The equivalent of `Zero` for hashes.
///
/// A hash that consists only of 0 bits is clear.
pub trait Clear {
    /// Returns `true` if the hash is clear.
    fn is_clear(&self) -> bool;

    /// Returns a clear hash.
    fn clear() -> Self;
}

impl Clear for [u8; 32] {
    fn is_clear(&self) -> bool {
        self.as_ref().iter().all(|&byte| byte == 0x00)
    }

    fn clear() -> Self {
        [0x00; 32]
    }
}

impl Clear for Hash {
    fn is_clear(&self) -> bool {
        <[u8; 32] as Clear>::is_clear(&self.0)
    }

    fn clear() -> Self {
        Self(<[u8; 32] as Clear>::clear())
    }
}