use super::constant::{SAPLING_BASE_POINT, SAPLING_REDJUBJUB_COFACTOR};
use super::hash::hash_to_scalar;
use super::signature::Signature;

use zero_bls12_381::Fr;
use zero_jubjub::{JubjubAffine, JubjubExtended};
use zkstd::behave::{CurveGroup, SigUtils};

#[derive(Clone, Copy, Debug)]
pub struct PublicKey(pub(crate) JubjubExtended);

impl SigUtils<32> for PublicKey {
    fn to_bytes(self) -> [u8; Self::LENGTH] {
        self.0.to_bytes()
    }

    fn from_bytes(bytes: [u8; Self::LENGTH]) -> Option<Self> {
        match JubjubExtended::from_bytes(bytes) {
            Some(point) => Some(Self(point)),
            None => None,
        }
    }
}

impl PublicKey {
    pub fn new(raw: JubjubExtended) -> Self {
        PublicKey(raw)
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
}
