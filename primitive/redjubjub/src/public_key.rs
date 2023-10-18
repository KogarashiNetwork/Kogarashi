use super::constant::{sapling_base_point, sapling_redjubjub_cofactor};
use super::hash::sapling_hash;
use super::signature::Signature;

use serde::{Deserialize, Serialize};
use zkstd::common::SigUtils;
use zkstd::common::*;

/// RedJubjub public key struct used for signature verification
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Decode,
    Encode,
)]
pub struct PublicKey<P: Pairing>(pub(crate) P::JubjubExtended);

impl<P: Pairing> SigUtils<32> for PublicKey<P> {
    fn to_bytes(self) -> [u8; 32] {
        self.0.to_bytes()
    }

    fn from_bytes(bytes: [u8; 32]) -> Option<Self> {
        P::JubjubExtended::from_bytes(bytes).map(Self)
    }
}

impl<P: Pairing> PublicKey<P> {
    pub fn new(raw: P::JubjubExtended) -> Self {
        PublicKey(raw)
    }

    pub fn zero() -> Self {
        Self(P::JubjubExtended::zero())
    }

    pub fn inner(&self) -> P::JubjubExtended {
        self.0
    }

    pub fn from_raw_bytes(bytes: &[u8]) -> Option<Self> {
        assert_eq!(bytes.len(), Self::LENGTH);
        let bytes: [u8; 32] = bytes[..32].try_into().unwrap();
        Self::from_bytes(bytes)
    }

    #[allow(non_snake_case)]
    pub fn validate(self, m: &[u8], sig: Signature) -> bool {
        // c = H(R||vk||m)
        let c = sapling_hash::<P::JubjubScalar>(&sig.r, &self.to_bytes(), m);

        let R = match P::JubjubAffine::from_bytes(sig.r) {
            Some(R) => R,
            None => return false,
        };
        let S = match P::JubjubScalar::from_bytes(sig.s) {
            Some(S) => S,
            None => return false,
        };

        // h_G(-S * P_G + R + c * vk)
        ((-(sapling_base_point::<P>() * S) + R + self.0 * c)
            * sapling_redjubjub_cofactor::<P::JubjubScalar>())
        .is_identity()
    }

    pub fn verify_simple_preaudit_deprecated<T>(
        &self,
        _ctx: &'static [u8],
        _msg: &[u8],
        _sig: &[u8],
    ) -> T {
        todo!()
    }

    pub fn randomize_public(&self, r: P::JubjubScalar) -> Self {
        Self(self.0 * r)
    }
}
