#![cfg_attr(not(feature = "std"), no_std)]

mod constant;
mod hash;
mod keyring;
mod private_key;
mod public_key;
mod signature;

pub use private_key::SecretKey;
pub use public_key::PublicKey;

use parity_scale_codec::alloc::string::String;
use parity_scale_codec::{Decode, Encode};
use sp_core::crypto::{
    CryptoType, CryptoTypeId, CryptoTypePublicPair, Derive, DeriveJunction, Pair as TraitPair,
    Public as TraitPublic, SecretStringError, UncheckedFrom,
};
use sp_runtime_interface::pass_by::PassByInner;
use sp_std::vec::Vec;

/// An identifier used to match public keys against redsa keys
pub const CRYPTO_ID: CryptoTypeId = CryptoTypeId(*b"reds");

type Seed = [u8; 32];

#[derive(Encode, Decode, PassByInner)]
pub struct Signature(pub [u8; 64]);

impl Signature {
    pub fn from_raw(data: [u8; 64]) -> Signature {
        Signature(data)
    }

    pub fn from_slice(data: &[u8]) -> Signature {
        let mut r = [0u8; 64];
        r.copy_from_slice(data);
        Signature(r)
    }
}

#[derive(Clone, Encode, Decode, PassByInner, PartialEq, Eq, Hash)]
pub struct Public(pub [u8; 32]);

impl Public {
    /// A new instance from the given 33-byte `data`.
    ///
    /// NOTE: No checking goes on to ensure this is a real public key. Only use it if
    /// you are certain that the array actually is a pubkey. GIGO!
    pub fn from_raw(data: [u8; 32]) -> Self {
        Self(data)
    }

    /// Return a slice filled with raw data.
    pub fn as_array_ref(&self) -> &[u8; 32] {
        self.as_ref()
    }
}

impl TraitPublic for Public {
    /// A new instance from the given slice that should be 33 bytes long.
    ///
    /// NOTE: No checking goes on to ensure this is a real public key. Only use it if
    /// you are certain that the array actually is a pubkey. GIGO!
    fn from_slice(data: &[u8]) -> Self {
        let mut r = [0u8; 32];
        r.copy_from_slice(data);
        Self(r)
    }

    fn to_public_crypto_pair(&self) -> CryptoTypePublicPair {
        CryptoTypePublicPair(CRYPTO_ID, self.to_raw_vec())
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

impl Derive for Public {}

impl Default for Public {
    fn default() -> Self {
        Public([0u8; 32])
    }
}

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

impl From<Pair> for Public {
    fn from(x: Pair) -> Self {
        x.public()
    }
}

impl UncheckedFrom<[u8; 32]> for Public {
    fn unchecked_from(x: [u8; 32]) -> Self {
        Public(x)
    }
}

impl sp_std::convert::TryFrom<&[u8]> for Public {
    type Error = ();

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() == 32 {
            Ok(Self::from_slice(data))
        } else {
            Err(())
        }
    }
}

pub enum DeriveError {
    /// A soft key was found in the path (and is unsupported).
    SoftKeyInPath,
}

/// A key pair.
#[derive(Clone)]
pub struct Pair {
    public: PublicKey,
    secret: SecretKey,
}

impl TraitPair for Pair {
    type Public = Public;
    type Seed = Seed;
    type Signature = Signature;
    type DeriveError = DeriveError;

    /// Generate new secure (random) key pair and provide the recovery phrase.
    ///
    /// You can recover the same key later with `from_phrase`.
    fn generate_with_phrase(password: Option<&str>) -> (Pair, String, Seed) {
        todo!()
    }

    /// Generate key pair from given recovery phrase and password.
    fn from_phrase(
        phrase: &str,
        password: Option<&str>,
    ) -> Result<(Pair, Seed), SecretStringError> {
        todo!()
    }

    /// Make a new key pair from secret seed material.
    ///
    /// You should never need to use this; generate(), generate_with_phrase
    fn from_seed(seed: &Seed) -> Pair {
        todo!()
    }

    /// Make a new key pair from secret seed material. The slice must be 32 bytes long or it
    /// will return `None`.
    ///
    /// You should never need to use this; generate(), generate_with_phrase
    fn from_seed_slice(seed_slice: &[u8]) -> Result<Pair, SecretStringError> {
        todo!()
    }

    /// Derive a child key from a series of given junctions.
    fn derive<Iter: Iterator<Item = DeriveJunction>>(
        &self,
        path: Iter,
        _seed: Option<Seed>,
    ) -> Result<(Pair, Option<Seed>), DeriveError> {
        todo!()
    }

    /// Get the public key.
    fn public(&self) -> Public {
        todo!()
    }

    /// Sign a message.
    fn sign(&self, message: &[u8]) -> Signature {
        todo!()
    }

    /// Verify a signature on a message. Returns true if the signature is good.
    fn verify<M: AsRef<[u8]>>(sig: &Self::Signature, message: M, pubkey: &Self::Public) -> bool {
        todo!()
    }

    /// Verify a signature on a message. Returns true if the signature is good.
    ///
    /// This doesn't use the type system to ensure that `sig` and `pubkey` are the correct
    /// size. Use it only if you're coming from byte buffers and need the speed.
    fn verify_weak<P: AsRef<[u8]>, M: AsRef<[u8]>>(sig: &[u8], message: M, pubkey: P) -> bool {
        todo!()
    }

    /// Return a vec filled with raw data.
    fn to_raw_vec(&self) -> Vec<u8> {
        todo!()
    }
}

impl CryptoType for Public {
    type Pair = Pair;
}

impl CryptoType for Signature {
    type Pair = Pair;
}

impl CryptoType for Pair {
    type Pair = Pair;
}

#[cfg(test)]
mod tests {
    use super::private_key::SecretKey;
    use rand_core::OsRng;
    use zero_jubjub::Fp;
    use zkstd::behave::Group;

    #[test]
    fn signature_test() {
        for _ in 0..1000 {
            let msg = b"test";
            let randomness = OsRng;
            let priv_key = SecretKey(Fp::random(OsRng));
            let sig = priv_key.sign(msg, randomness);

            let pub_key = priv_key.to_public_key();

            assert!(pub_key.validate(msg, sig))
        }
    }
}