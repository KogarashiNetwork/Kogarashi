#[cfg(feature = "full_crypto")]
use crate::Pair;

use codec::{Decode, Encode};
use sp_core::{crypto::CryptoType, hash::H512};
use sp_runtime_interface::pass_by::PassByInner;

#[cfg(feature = "std")]
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
#[cfg(feature = "std")]
use sp_core::hexdisplay::HexDisplay;

/// A signature (a 512-bit value).
#[derive(Encode, Decode, PassByInner)]
pub struct Signature(pub [u8; 64]);

impl sp_std::convert::TryFrom<&[u8]> for Signature {
    type Error = ();

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() == 64 {
            let mut inner = [0u8; 64];
            inner.copy_from_slice(data);
            Ok(Signature(inner))
        } else {
            Err(())
        }
    }
}

#[cfg(feature = "std")]
impl Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&hex::encode(self))
    }
}

#[cfg(feature = "std")]
impl<'de> Deserialize<'de> for Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let signature_hex = hex::decode(&String::deserialize(deserializer)?)
            .map_err(|e| de::Error::custom(format!("{:?}", e)))?;
        Ok(Signature::try_from(signature_hex.as_ref())
            .map_err(|e| de::Error::custom(format!("{:?}", e)))?)
    }
}

impl Clone for Signature {
    fn clone(&self) -> Self {
        let mut r = [0u8; 64];
        r.copy_from_slice(&self.0[..]);
        Signature(r)
    }
}

impl Default for Signature {
    fn default() -> Self {
        Signature([0u8; 64])
    }
}

impl PartialEq for Signature {
    fn eq(&self, b: &Self) -> bool {
        self.0[..] == b.0[..]
    }
}

impl Eq for Signature {}

impl From<Signature> for H512 {
    fn from(v: Signature) -> H512 {
        H512::from(v.0)
    }
}

impl From<Signature> for [u8; 64] {
    fn from(v: Signature) -> [u8; 64] {
        v.0
    }
}

impl AsRef<[u8; 64]> for Signature {
    fn as_ref(&self) -> &[u8; 64] {
        &self.0
    }
}

impl AsRef<[u8]> for Signature {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl AsMut<[u8]> for Signature {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0[..]
    }
}

impl sp_std::fmt::Debug for Signature {
    #[cfg(feature = "std")]
    fn fmt(&self, f: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
        write!(f, "{}", HexDisplay::from(&self.0))
    }

    #[cfg(not(feature = "std"))]
    fn fmt(&self, _: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
        Ok(())
    }
}

#[cfg(feature = "full_crypto")]
impl sp_std::hash::Hash for Signature {
    fn hash<H: sp_std::hash::Hasher>(&self, state: &mut H) {
        sp_std::hash::Hash::hash(&self.0[..], state);
    }
}

impl Signature {
    /// A new instance from the given 64-byte `data`.
    ///
    /// NOTE: No checking goes on to ensure this is a real signature. Only use it if
    /// you are certain that the array actually is a signature. GIGO!
    pub fn from_raw(data: [u8; 64]) -> Signature {
        Signature(data)
    }

    /// A new instance from the given slice that should be 64 bytes long.
    ///
    /// NOTE: No checking goes on to ensure this is a real signature. Only use it if
    /// you are certain that the array actually is a signature. GIGO!
    pub fn from_slice(data: &[u8]) -> Self {
        let mut r = [0u8; 64];
        r.copy_from_slice(data);
        Signature(r)
    }

    /// A new instance from an H512.
    ///
    /// NOTE: No checking goes on to ensure this is a real signature. Only use it if
    /// you are certain that the array actually is a signature. GIGO!
    pub fn from_h512(v: H512) -> Signature {
        Signature(v.into())
    }
}

impl CryptoType for Signature {
    #[cfg(feature = "full_crypto")]
    type Pair = Pair;
}
