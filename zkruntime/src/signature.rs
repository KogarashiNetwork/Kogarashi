use codec::{Decode, Encode};
#[cfg(feature = "std")]
pub use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sp_core::RuntimeDebug;
use sp_std::convert::TryFrom;

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Eq, PartialEq, Clone, Encode, Decode, RuntimeDebug)]
pub enum MultiSignature {
    /// An Redjubjub signature.
    Redjubjub(red_jubjub::Signature),
}

impl From<red_jubjub::Signature> for MultiSignature {
    fn from(x: red_jubjub::Signature) -> Self {
        MultiSignature::Redjubjub(x)
    }
}

impl TryFrom<MultiSignature> for red_jubjub::Signature {
    type Error = ();
    fn try_from(m: MultiSignature) -> Result<Self, Self::Error> {
        if let MultiSignature::Redjubjub(x) = m {
            Ok(x)
        } else {
            Err(())
        }
    }
}

impl Default for MultiSignature {
    fn default() -> Self {
        MultiSignature::Redjubjub(Default::default())
    }
}

/// Public key for any known crypto algorithm.
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Encode, Decode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum MultiSigner {
    /// An Redjubjub identity.
    Redjubjub(redjubjub::Public),
}
