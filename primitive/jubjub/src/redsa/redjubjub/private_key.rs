use super::hash::hash_to_scalar;
use super::public_key::PublicKey;
use super::signature::Signature;
use crate::curve::JubjubExtended;

use rand_core::RngCore;
use zero_bls12_381::Fr;
use zkstd::behave::{CurveGroup, SigUtils};

#[derive(Clone)]
pub struct SecretKey(pub(crate) Fr);

impl SecretKey {
    #[allow(non_snake_case)]
    pub fn sign(self, m: &[u8], mut rand: impl RngCore) -> Signature {
        // T uniformly at random
        let mut T = [0u8; 80];
        rand.fill_bytes(&mut T[..]);

        // r = H(T||M)
        let r = hash_to_scalar(&T, m);

        // R = r * P_G
        let R = (JubjubExtended::ADDITIVE_GENERATOR * r).to_bytes();

        // S = r + H(R||m)
        let S = r + hash_to_scalar(&R, m);

        Signature::new(R, S.to_bytes())
    }

    pub fn to_public_key(&self) -> PublicKey {
        PublicKey(self.0 * JubjubExtended::ADDITIVE_GENERATOR)
    }
}
