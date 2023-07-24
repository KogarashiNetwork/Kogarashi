use super::constant::{CRYPTO_ID, SAPLING_BASE_POINT, SAPLING_REDJUBJUB_COFACTOR};
use super::hash::hash_to_scalar;
use super::signature::Signature;
use super::Pair;

use zero_bls12_381::Fr;
use zero_jubjub::{JubjubAffine, JubjubExtended};
use zkstd::behave::{CurveGroup, SigUtils};

use parity_scale_codec::{Decode, Encode};
use schnorrkel::SignatureResult;
use sp_core::crypto::{
    CryptoType, CryptoTypePublicPair, Derive, Pair as TraitPair, Public as TraitPublic,
    UncheckedFrom,
};
use sp_runtime_interface::pass_by::PassByInner;

#[derive(Clone, Copy, Debug)]
pub struct PublicKey(pub(crate) JubjubExtended);

impl SigUtils<32> for PublicKey {
    fn to_bytes(self) -> [u8; Self::LENGTH] {
        self.0.to_bytes()
    }

    fn from_bytes(bytes: [u8; Self::LENGTH]) -> Option<Self> {
        JubjubExtended::from_bytes(bytes).map(Self)
    }
}

impl PublicKey {
    pub fn new(raw: JubjubExtended) -> Self {
        PublicKey(raw)
    }

    pub fn from_raw_bytes(bytes: &[u8]) -> Option<Self> {
        assert_eq!(bytes.len(), Self::LENGTH);
        let bytes: [u8; Self::LENGTH] = bytes[..32].try_into().unwrap();
        Self::from_bytes(bytes)
    }

    #[allow(non_snake_case)]
    pub fn validate(self, m: &[u8], sig: Signature) -> bool {
        // c = H(R||vk||m)
        let c = hash_to_scalar(&sig.r, &self.to_bytes(), m);

        let R = match JubjubAffine::from_bytes(sig.r) {
            Some(R) => R,
            None => return false,
        };
        let S = match Fr::from_bytes(sig.s) {
            Some(S) => S,
            None => return false,
        };

        // h_G(-S * P_G + R + c * vk)
        (SAPLING_REDJUBJUB_COFACTOR * (-(S * SAPLING_BASE_POINT) + R + c * self.0)).is_identity()
    }

    #[allow(non_snake_case)]
    pub fn verify_simple_preaudit_deprecated(
        &self,
        _ctx: &'static [u8],
        _msg: &[u8],
        _sig: &[u8],
    ) -> SignatureResult<()> {
        todo!()
    }
}

#[derive(Clone, Debug, Default, Decode, Encode, PassByInner, PartialEq, Eq, Hash)]
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

impl CryptoType for Public {
    type Pair = Pair;
}
