use super::constant::{SAPLING_BASE_POINT, SAPLING_REDJUBJUB_COFACTOR};
use super::hash::sapling_hash;
use super::signature::Signature;

use bls_12_381::Fr;
use jub_jub::{Fp, JubjubAffine, JubjubExtended};
use zkstd::behave::{CurveGroup, SigUtils};

/// RedJubjub public key struct used for signature verification
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
        let c = sapling_hash(&sig.r, &self.to_bytes(), m);

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

    pub fn verify_simple_preaudit_deprecated<T>(
        &self,
        _ctx: &'static [u8],
        _msg: &[u8],
        _sig: &[u8],
    ) -> T {
        todo!()
    }

    pub fn randomize_public(&self, r: Fp) -> Self {
        Self(self.0 * r)
    }
}
