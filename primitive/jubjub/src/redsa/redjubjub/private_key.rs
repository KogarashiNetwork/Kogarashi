use crate::fp::Fp;

use super::constant::SAPLING_BASE_POINT;
use super::hash::hash_to_scalar;
use super::public_key::PublicKey;
use super::signature::Signature;

use rand_core::RngCore;
use zkstd::behave::SigUtils;

#[derive(Clone, Debug)]
pub struct SecretKey(pub(crate) Fp);

impl SecretKey {
    #[allow(non_snake_case)]
    pub fn sign(self, m: &[u8], mut rand: impl RngCore) -> Signature {
        // T uniformly at random
        let mut T = [0u8; 80];
        rand.fill_bytes(&mut T[..]);

        // r = H(T||vk||M)
        let pk = self.to_public_key();
        let r = hash_to_scalar(&T, &pk.to_bytes(), m);

        // R = r * P_G
        let R = (SAPLING_BASE_POINT * r).to_bytes();

        // S = r + H(R||m)
        let S = (r + hash_to_scalar(&R, &pk.to_bytes(), m)).to_bytes();

        Signature::new(R, S)
    }

    pub fn to_public_key(&self) -> PublicKey {
        PublicKey(SAPLING_BASE_POINT * self.0)
    }
}
