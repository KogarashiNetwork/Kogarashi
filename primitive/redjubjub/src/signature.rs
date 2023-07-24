use super::Pair;

use parity_scale_codec::{Decode, Encode};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use sp_core::crypto::CryptoType;
use sp_runtime_interface::pass_by::PassByInner;
use zkstd::behave::SigUtils;

#[derive(Clone)]
pub struct Signature {
    pub(crate) r: [u8; 32],
    pub(crate) s: [u8; 32],
}

impl SigUtils<64> for Signature {
    fn from_bytes(bytes: [u8; Self::LENGTH]) -> Option<Self> {
        let mut r = [0u8; 32];
        let mut s = [0u8; 32];
        r.copy_from_slice(&bytes[..32]);
        s.copy_from_slice(&bytes[32..]);
        Some(Self { r, s })
    }

    fn to_bytes(self) -> [u8; Self::LENGTH] {
        let mut bytes = [0u8; 64];
        bytes.copy_from_slice(&self.r);
        bytes.copy_from_slice(&self.s);
        bytes
    }
}

impl Signature {
    pub(crate) fn new(r: [u8; 32], s: [u8; 32]) -> Self {
        Self { r, s }
    }

    pub fn from_raw_bytes(bytes: &[u8]) -> Option<Self> {
        assert_eq!(bytes.len(), Self::LENGTH);
        let bytes: [u8; Self::LENGTH] = bytes[..64].try_into().unwrap();
        Self::from_bytes(bytes)
    }
}

#[derive(Debug, Decode, Encode, PassByInner)]
pub struct Sig(pub [u8; 64]);

impl sp_std::convert::TryFrom<&[u8]> for Sig {
    type Error = ();

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() == 64 {
            let mut inner = [0u8; 64];
            inner.copy_from_slice(data);
            Ok(Sig(inner))
        } else {
            Err(())
        }
    }
}

impl Sig {
    pub fn from_raw(data: [u8; 64]) -> Sig {
        Sig(data)
    }

    pub fn from_slice(data: &[u8]) -> Sig {
        let mut r = [0u8; 64];
        r.copy_from_slice(data);
        Sig(r)
    }
}

impl Serialize for Sig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&hex::encode(self))
    }
}

impl<'de> Deserialize<'de> for Sig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let signature_hex = hex::decode(String::deserialize(deserializer)?)
            .map_err(|e| de::Error::custom(format!("{:?}", e)))?;
        Sig::try_from(signature_hex.as_ref()).map_err(|e| de::Error::custom(format!("{:?}", e)))
    }
}

impl AsRef<[u8]> for Sig {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl AsMut<[u8]> for Sig {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0[..]
    }
}

impl CryptoType for Signature {
    type Pair = Pair;
}
