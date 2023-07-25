use super::constant::SAPLING_BASE_POINT;
use super::hash::sapling_hash;
use super::public_key::PublicKey;
use super::signature::Signature;

use jub_jub::Fp;
use rand_core::RngCore;
use zkstd::behave::SigUtils;

#[derive(Clone, Copy, Debug)]
pub struct SecretKey(pub(crate) Fp);

impl SigUtils<32> for SecretKey {
    fn from_bytes(bytes: [u8; 32]) -> Option<Self> {
        Fp::from_bytes(bytes).map(Self)
    }

    fn to_bytes(self) -> [u8; Self::LENGTH] {
        self.0.to_bytes()
    }
}

impl SecretKey {
    pub fn new(key: Fp) -> Self {
        Self(key)
    }

    pub fn from_raw_bytes(bytes: &[u8]) -> Option<Self> {
        assert_eq!(bytes.len(), Self::LENGTH);
        let bytes: [u8; Self::LENGTH] = bytes[..].try_into().unwrap();
        Self::from_bytes(bytes)
    }

    #[allow(non_snake_case)]
    pub fn sign(&self, m: &[u8], mut rand: impl RngCore) -> Signature {
        // T uniformly at random
        let mut T = [0u8; 80];
        rand.fill_bytes(&mut T[..]);

        // r = H(T||vk||M)
        let pk = self.to_public_key();
        let r = sapling_hash(&T, &pk.to_bytes(), m);

        // R = r * P_G
        let R = (r * SAPLING_BASE_POINT).to_bytes();

        // S = r + H(R||m) * sk
        let S = (r + sapling_hash(&R, &pk.to_bytes(), m) * self.0).to_bytes();

        Signature::new(R, S)
    }

    pub fn to_public_key(&self) -> PublicKey {
        PublicKey(SAPLING_BASE_POINT * self.0)
    }
}
