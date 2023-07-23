use super::hash::hash_to_scalar;
use super::signature::Signature;
use crate::{curve::JubjubExtended, JubjubAffine};

use zero_bls12_381::Fr;
use zkstd::behave::{CurveGroup, PrimeField, SigUtils};

#[derive(Clone)]
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
        // c = H(R||m)
        let c = hash_to_scalar(&sig.r, m);

        let R = match JubjubAffine::from_bytes(sig.r) {
            Some(R) => R,
            None => return false,
        };
        let S = match Fr::from_bytes(sig.s) {
            Some(S) => S,
            None => return false,
        };

        // rejubjub cofactor
        let h_G = Fr::one().double().double().double();

        // h_G(-S * P_G + R + c * vk)
        (h_G * (-S * JubjubExtended::ADDITIVE_GENERATOR + R + c * self.0)).is_identity()
    }
}
