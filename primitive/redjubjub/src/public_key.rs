use super::constant::{sapling_base_point, sapling_redjubjub_cofactor};
use super::hash::sapling_hash;
use super::signature::Signature;

use serde::{Deserialize, Serialize};
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
pub struct PublicKey<P: RedDSA>(pub(crate) P::Extended);

impl<P: RedDSA> SigUtils<32> for PublicKey<P> {
    fn to_bytes(self) -> [u8; 32] {
        self.0.to_bytes()
    }

    fn from_bytes(bytes: [u8; 32]) -> Option<Self> {
        P::Extended::from_bytes(bytes).map(Self)
    }
}

impl<P: RedDSA> PublicKey<P> {
    pub fn new(raw: P::Extended) -> Self {
        PublicKey(raw)
    }

    pub fn zero() -> Self {
        Self(P::Extended::zero())
    }

    pub fn inner(&self) -> P::Extended {
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
        let c = sapling_hash::<P::Scalar>(&sig.r, &self.to_bytes(), m);

        let R = match P::Affine::from_bytes(sig.r) {
            Some(R) => R,
            None => return false,
        };
        let S = match P::Scalar::from_bytes(sig.s) {
            Some(S) => S,
            None => return false,
        };

        // h_G(-S * P_G + R + c * vk)
        ((-(sapling_base_point::<P>() * S) + R + self.0 * c)
            * sapling_redjubjub_cofactor::<P::Scalar>())
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

    pub fn randomize_public(&self, r: P::Scalar) -> Self {
        Self(self.0 * r)
    }
}
