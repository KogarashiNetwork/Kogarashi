#[cfg_attr(not(feature = "std"), no_std)]
use codec::{Decode, Encode};
use rand_core::SeedableRng;
use rand_xorshift::XorShiftRng;
use sp_core::crypto::{
    CryptoType, CryptoTypeId, CryptoTypePublicPair, Derive, Public as TraitPublic, UncheckedFrom,
};
use sp_core::hash::{H256, H512};
use sp_runtime_interface::pass_by::PassByInner;
use sp_std::ops::Deref;

#[cfg(feature = "std")]
use bip39::{Language, Mnemonic, MnemonicType};
#[cfg(feature = "full_crypto")]
use schnorrkel::SignatureResult;
#[cfg(feature = "std")]
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
#[cfg(feature = "full_crypto")]
use sp_core::crypto::{DeriveJunction, Pair as TraitPair, PublicError, SecretStringError};
#[cfg(feature = "std")]
use sp_core::{crypto::Ss58Codec, hexdisplay::HexDisplay};
#[cfg(feature = "std")]
use substrate_bip39::mini_secret_from_entropy;

#[cfg(feature = "full_crypto")]
use red_jubjub::{kogarashi_hash, Keypair, PublicKey, SecretKey};
#[cfg(feature = "full_crypto")]
use zkstd::behave::SigUtils;

// signing context
pub(crate) const SIGNING_CTX: &[u8] = b"substrate";

/// An identifier used to match public keys against redjubjub keys
pub const CRYPTO_ID: CryptoTypeId = CryptoTypeId(*b"redj");

/// A secret seed. It's not called a "secret key" because ring doesn't expose the secret keys
/// of the key pair (yeah, dumb); as such we're forced to remember the seed manually if we
/// will need it later (such as for HDKD).
#[cfg(feature = "full_crypto")]
type Seed = [u8; 32];

/// A public key.
#[cfg_attr(feature = "full_crypto", derive(Hash))]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Encode, Decode, Default, PassByInner)]
pub struct Public(pub [u8; 32]);

/// A key pair.
#[cfg(feature = "full_crypto")]
pub struct Pair(Keypair);

#[cfg(feature = "full_crypto")]
impl Clone for Pair {
    fn clone(&self) -> Self {
        Pair(Keypair {
            public: self.0.public.clone(),
            secret: SecretKey::from_bytes(self.0.secret.to_bytes())
                .expect("key is always the correct size; qed"),
        })
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

/// A localized signature also contains sender information.
#[cfg(feature = "std")]
#[derive(PartialEq, Eq, Clone, Debug, Encode, Decode)]
pub struct LocalizedSignature {
    /// The signer of the signature.
    pub signer: Public,
    /// The signature itself.
    pub signature: Signature,
}

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
/// Derive a single hard junction.
#[cfg(feature = "full_crypto")]
fn derive_hard_junction(secret_seed: &Seed, cc: &[u8; 32]) -> Seed {
    ("RedjubjubHDKD", secret_seed, cc).using_encoded(|data| {
        let mut res = [0u8; 32];
        res.copy_from_slice(blake2_rfc::blake2b::blake2b(32, &[], data).as_bytes());
        res
    })
}

/// An error when deriving a key.
#[cfg(feature = "full_crypto")]
pub enum DeriveError {
    /// A soft key was found in the path (and is unsupported).
    SoftKeyInPath,
}

#[cfg(feature = "full_crypto")]
impl TraitPair for Pair {
    type Public = Public;
    type Seed = Seed;
    type Signature = Signature;
    type DeriveError = DeriveError;

    /// Generate new secure (random) key pair and provide the recovery phrase.
    ///
    /// You can recover the same key later with `from_phrase`.
    fn generate_with_phrase(password: Option<&str>) -> (Pair, String, Seed) {
        let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
        let phrase = mnemonic.phrase();
        let (pair, seed) = Self::from_phrase(phrase, password)
            .expect("All phrases generated by Mnemonic are valid; qed");
        (pair, phrase.to_owned(), seed)
    }

    /// Generate key pair from given recovery phrase and password.
    fn from_phrase(
        phrase: &str,
        password: Option<&str>,
    ) -> Result<(Pair, Seed), SecretStringError> {
        Mnemonic::from_phrase(phrase, Language::English)
            .map_err(|_| SecretStringError::InvalidPhrase)
            .map(|m| Self::from_entropy(m.entropy(), password))
    }

    /// Make a new key pair from secret seed material.
    ///
    /// You should never need to use this; generate(), generate_with_phrase
    fn from_seed(seed: &Seed) -> Pair {
        Self::from_seed_slice(&seed[..]).expect("seed has valid length; qed")
    }

    /// Make a new key pair from secret seed material. The slice must be 32 bytes long or it
    /// will return `None`.
    ///
    /// You should never need to use this; generate(), generate_with_phrase
    fn from_seed_slice(seed_slice: &[u8]) -> Result<Pair, SecretStringError> {
        let secret = SecretKey::new(kogarashi_hash(seed_slice));
        Ok(Self(Keypair::new(secret)))
    }

    /// Derive a child key from a series of given junctions.
    fn derive<Iter: Iterator<Item = DeriveJunction>>(
        &self,
        path: Iter,
        _seed: Option<Seed>,
    ) -> Result<(Pair, Option<Seed>), DeriveError> {
        let mut acc = self.0.secret.to_bytes();
        for j in path {
            match j {
                DeriveJunction::Soft(_cc) => return Err(DeriveError::SoftKeyInPath),
                DeriveJunction::Hard(cc) => acc = derive_hard_junction(&acc, &cc),
            }
        }
        Ok((Self::from_seed(&acc), Some(acc)))
    }

    /// Get the public key.
    fn public(&self) -> Public {
        Public(self.0.public.to_bytes())
    }

    /// Sign a message.
    fn sign(&self, message: &[u8]) -> Signature {
        let mut rng = XorShiftRng::from_seed([
            0xbc, 0x4f, 0x6d, 0x44, 0xd6, 0x2f, 0x27, 0x6c, 0xb9, 0x63, 0xaf, 0xd0, 0x54, 0x55,
            0x86, 0x3d,
        ]);
        let r = self.0.secret.sign(message, &mut rng);
        Signature(r.to_bytes())
    }

    /// Verify a signature on a message. Returns true if the signature is good.
    fn verify<M: AsRef<[u8]>>(sig: &Self::Signature, message: M, pubkey: &Self::Public) -> bool {
        Self::verify_weak(&sig.0[..], message.as_ref(), pubkey)
    }

    /// Verify a signature on a message. Returns true if the signature is good.
    ///
    /// This doesn't use the type system to ensure that `sig` and `pubkey` are the correct
    /// size. Use it only if you're coming from byte buffers and need the speed.
    fn verify_weak<P: AsRef<[u8]>, M: AsRef<[u8]>>(sig: &[u8], message: M, pubkey: P) -> bool {
        let public_key = match PublicKey::from_raw_bytes(pubkey.as_ref()) {
            Some(pk) => pk,
            None => return false,
        };

        let sig = match red_jubjub::Signature::from_raw_bytes(sig) {
            Some(s) => s,
            None => return false,
        };

        public_key.validate(message.as_ref(), sig)
    }

    /// Return a vec filled with raw data.
    fn to_raw_vec(&self) -> Vec<u8> {
        self.0.secret.to_bytes().to_vec()
    }
}

#[cfg(feature = "full_crypto")]
impl Pair {
    /// Get the seed for this key.
    pub fn seed(&self) -> Seed {
        self.0.secret.to_bytes()
    }

    /// Exactly as `from_string` except that if no matches are found then, the the first 32
    /// characters are taken (padded with spaces as necessary) and used as the MiniSecretKey.
    #[cfg(feature = "std")]
    pub fn from_legacy_string(s: &str, password_override: Option<&str>) -> Pair {
        Self::from_string(s, password_override).unwrap_or_else(|_| {
            let mut padded_seed: Seed = [' ' as u8; 32];
            let len = s.len().min(32);
            padded_seed[..len].copy_from_slice(&s.as_bytes()[..len]);
            Self::from_seed(&padded_seed)
        })
    }
    /// Make a new key pair from binary data derived from a valid seed phrase.
    ///
    /// This uses a key derivation function to convert the entropy into a seed, then returns
    /// the pair generated from it.
    pub fn from_entropy(entropy: &[u8], password: Option<&str>) -> (Pair, Seed) {
        let seed = mini_secret_from_entropy(entropy, password.unwrap_or(""))
            .expect("32 bytes can always build a key; qed")
            .to_bytes();

        let secret = SecretKey::new(kogarashi_hash(&seed));
        (Self(Keypair::new(secret)), secret.to_bytes())
    }

    /// Verify a signature on a message. Returns `true` if the signature is good.
    /// Supports old 0.1.1 deprecated signatures and should be used only for backward
    /// compatibility.
    pub fn verify_deprecated<M: AsRef<[u8]>>(sig: &Signature, message: M, pubkey: &Public) -> bool {
        // Match both schnorrkel 0.1.1 and 0.8.0+ signatures, supporting both wallets
        // that have not been upgraded and those that have.
        match PublicKey::from_bytes(*pubkey.as_ref()) {
            Some(pk) => pk
                .verify_simple_preaudit_deprecated::<SignatureResult<()>>(
                    SIGNING_CTX,
                    message.as_ref(),
                    &sig.0[..],
                )
                .is_ok(),
            None => false,
        }
    }
}

impl CryptoType for Public {
    #[cfg(feature = "full_crypto")]
    type Pair = Pair;
}

impl CryptoType for Signature {
    #[cfg(feature = "full_crypto")]
    type Pair = Pair;
}

#[cfg(feature = "full_crypto")]
impl CryptoType for Pair {
    type Pair = Pair;
}
