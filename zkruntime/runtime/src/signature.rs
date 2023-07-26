use codec::{Decode, Encode};
use redjubjub::{Public, Signature};
use sp_core::RuntimeDebug;
use sp_runtime::traits::{Lazy, Verify};
use sp_std::convert::TryFrom;

#[cfg(feature = "std")]
pub use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Eq, PartialEq, Clone, Encode, Decode, RuntimeDebug)]
pub enum MultiSignature {
    /// An Redjubjub signature.
    Redjubjub(Signature),
}

impl From<Signature> for MultiSignature {
    fn from(x: Signature) -> Self {
        MultiSignature::Redjubjub(x)
    }
}

impl TryFrom<MultiSignature> for Signature {
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
    Redjubjub(Public),
}
