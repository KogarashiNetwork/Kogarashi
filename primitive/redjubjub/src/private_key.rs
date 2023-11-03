use super::hash::sapling_hash;
use super::public_key::PublicKey;
use super::signature::Signature;
use crate::constant::sapling_base_point;

use rand_core::RngCore;
use zkstd::common::{FftField, RedDSA, SigUtils};

/// RedJubjub secret key struct used for signing transactions
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecretKey<P: RedDSA>(pub(crate) P::Scalar);

impl<P: RedDSA> SigUtils<32> for SecretKey<P> {
    fn from_bytes(bytes: [u8; 32]) -> Option<Self> {
        P::Scalar::from_bytes(bytes).map(Self::new)
    }

    fn to_bytes(self) -> [u8; 32] {
        self.0.to_bytes()
    }
}

impl<P: RedDSA> SecretKey<P> {
    pub fn new(key: P::Scalar) -> Self {
        Self(key)
    }

    pub fn from_seed(seed: [u8; 32]) -> Option<Self> {
        Self::from_bytes(seed)
    }

    pub fn from_raw_bytes(raw_bytes: &[u8]) -> Option<Self> {
        let mut raw_bytes = raw_bytes.to_vec();
        if raw_bytes.len() < 64 {
            raw_bytes.resize(64, 0);
        }
        let bytes: [u8; 64] = raw_bytes[..64].try_into().unwrap();
        Some(Self(P::Scalar::from_bytes_wide(&bytes)))
    }

    #[allow(non_snake_case)]
    pub fn sign(&self, m: &[u8], mut rand: impl RngCore) -> Signature {
        // T uniformly at random
        let mut T = [0u8; 80];
        rand.fill_bytes(&mut T[..]);

        // r = H(T||vk||M)
        let pk = self.to_public_key();
        let r = sapling_hash::<P::Scalar>(&T, &pk.to_bytes(), m);

        // R = r * P_G
        let R = (sapling_base_point::<P>() * r).to_bytes();

        // S = r + H(R||m) * sk
        let S = (r + sapling_hash::<P::Scalar>(&R, &pk.to_bytes(), m) * self.0).to_bytes();

        Signature::new(R, S)
    }

    pub fn to_public_key(&self) -> PublicKey<P> {
        PublicKey(sapling_base_point::<P>() * self.0)
    }

    pub fn randomize_private(&self, r: P::Scalar) -> Self {
        Self(r * self.0)
    }
}
