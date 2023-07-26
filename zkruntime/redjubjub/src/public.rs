use crate::{Pair, CRYPTO_ID};

use codec::{Decode, Encode};
use sp_core::crypto::{
    CryptoType, CryptoTypePublicPair, Derive, Public as TraitPublic, UncheckedFrom,
};
use sp_core::hash::H256;
use sp_runtime_interface::pass_by::PassByInner;
use sp_std::ops::Deref;

#[cfg(feature = "std")]
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
#[cfg(feature = "std")]
use sp_core::{crypto::Ss58Codec, hexdisplay::HexDisplay};

#[cfg(feature = "full_crypto")]
use sp_core::crypto::{Pair as TraitPair, PublicError};

/// A public key.
#[cfg_attr(feature = "full_crypto", derive(Hash))]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Encode, Decode, Default, PassByInner)]
pub struct Public(pub [u8; 32]);

impl Public {
    /// A new instance from the given 32-byte `data`.
    ///
    /// NOTE: No checking goes on to ensure this is a real public key. Only use it if
    /// you are certain that the array actually is a pubkey. GIGO!
    pub fn from_raw(data: [u8; 32]) -> Self {
        Public(data)
    }

    /// A new instance from an H256.
    ///
    /// NOTE: No checking goes on to ensure this is a real public key. Only use it if
    /// you are certain that the array actually is a pubkey. GIGO!
    pub fn from_h256(x: H256) -> Self {
        Public(x.into())
    }

    /// Return a slice filled with raw data.
    pub fn as_array_ref(&self) -> &[u8; 32] {
        self.as_ref()
    }
}

impl TraitPublic for Public {
    /// A new instance from the given slice that should be 32 bytes long.
    ///
    /// NOTE: No checking goes on to ensure this is a real public key. Only use it if
    /// you are certain that the array actually is a pubkey. GIGO!
    fn from_slice(data: &[u8]) -> Self {
        let mut r = [0u8; 32];
        r.copy_from_slice(data);
        Public(r)
    }

    fn to_public_crypto_pair(&self) -> CryptoTypePublicPair {
        CryptoTypePublicPair(CRYPTO_ID, self.to_raw_vec())
    }
}

impl Derive for Public {}

impl AsRef<[u8; 32]> for Public {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl AsRef<[u8]> for Public {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl AsMut<[u8]> for Public {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0[..]
    }
}

impl Deref for Public {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl sp_std::convert::TryFrom<&[u8]> for Public {
    type Error = ();

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() == 32 {
            let mut inner = [0u8; 32];
            inner.copy_from_slice(data);
            Ok(Public(inner))
        } else {
            Err(())
        }
    }
}

impl From<Public> for [u8; 32] {
    fn from(x: Public) -> Self {
        x.0
    }
}

#[cfg(feature = "full_crypto")]
impl From<Pair> for Public {
    fn from(x: Pair) -> Self {
        x.public()
    }
}

impl From<Public> for H256 {
    fn from(x: Public) -> Self {
        x.0.into()
    }
}

#[cfg(feature = "std")]
impl std::str::FromStr for Public {
    type Err = PublicError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_ss58check(s)
    }
}

impl UncheckedFrom<[u8; 32]> for Public {
    fn unchecked_from(x: [u8; 32]) -> Self {
        Public::from_raw(x)
    }
}

impl UncheckedFrom<H256> for Public {
    fn unchecked_from(x: H256) -> Self {
        Public::from_h256(x)
    }
}

#[cfg(feature = "std")]
impl std::fmt::Display for Public {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_ss58check())
    }
}

impl sp_std::fmt::Debug for Public {
    #[cfg(feature = "std")]
    fn fmt(&self, f: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
        let s = self.to_ss58check();
        write!(f, "{} ({}...)", HexDisplay::from(&self.0), &s[0..8])
    }

    #[cfg(not(feature = "std"))]
    fn fmt(&self, _: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
        Ok(())
    }
}

#[cfg(feature = "std")]
impl Serialize for Public {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_ss58check())
    }
}

#[cfg(feature = "std")]
impl<'de> Deserialize<'de> for Public {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Public::from_ss58check(&String::deserialize(deserializer)?)
            .map_err(|e| de::Error::custom(format!("{:?}", e)))
    }
}

impl From<Public> for CryptoTypePublicPair {
    fn from(key: Public) -> Self {
        (&key).into()
    }
}

impl From<&Public> for CryptoTypePublicPair {
    fn from(key: &Public) -> Self {
        CryptoTypePublicPair(CRYPTO_ID, key.to_raw_vec())
    }
}

impl CryptoType for Public {
    #[cfg(feature = "full_crypto")]
    type Pair = Pair;
}
