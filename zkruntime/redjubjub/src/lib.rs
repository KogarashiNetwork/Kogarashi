#![cfg_attr(not(feature = "std"), no_std)]

mod pair;
mod public;
mod signature;

use codec::{Decode, Encode};
use sp_core::crypto::CryptoTypeId;

pub use pair::Pair;
pub use public::Public;
pub use signature::Signature;

// signing context
pub(crate) const SIGNING_CTX: &[u8] = b"substrate";

/// An identifier used to match public keys against redjubjub keys
pub const CRYPTO_ID: CryptoTypeId = CryptoTypeId(*b"redj");

/// A secret seed. It's not called a "secret key" because ring doesn't expose the secret keys
/// of the key pair (yeah, dumb); as such we're forced to remember the seed manually if we
/// will need it later (such as for HDKD).
#[cfg(feature = "full_crypto")]
type Seed = [u8; 32];

/// A localized signature also contains sender information.
#[cfg(feature = "std")]
#[derive(PartialEq, Eq, Clone, Debug, Encode, Decode)]
pub struct LocalizedSignature {
    /// The signer of the signature.
    pub signer: Public,
    /// The signature itself.
    pub signature: Signature,
}
