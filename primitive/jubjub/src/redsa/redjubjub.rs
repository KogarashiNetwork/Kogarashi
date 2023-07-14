use crate::curve::JubjubExtend;
use crate::fp::Fp;

use dusk_bytes::Serializable;
use zero_bls12_381::Fr;
use zero_crypto::behave::DigitalSig;

use blake2b_simd::Params;
use rand_core::RngCore;

#[derive(Clone)]
pub struct Signature {
    s: [u8; 32],
    e: [u8; 32],
}

impl Signature {
    fn new(s: [u8; 32], e: [u8; 32]) -> Self {
        Self { s, e }
    }
}

#[derive(Clone)]
pub struct PublicKey(JubjubExtend);

impl DigitalSig for PublicKey {
    const LENGTH: usize = 32;
}

impl PublicKey {
    pub fn new(raw: JubjubExtend) -> Self {
        PublicKey(raw)
    }

    pub fn as_bytes(self) -> [u8; Self::LENGTH] {
        self.0.as_bytes()
    }
}

#[derive(Clone)]
pub struct SecretKey(Fp);

impl SecretKey {
    pub fn sign(self, m: &[u8], mut rand: impl RngCore) -> Signature {
        let mut t = [0u8; 80];
        rand.fill_bytes(&mut t[..]);
        let r = hash_to_scalar(&t, m);
        let R = (JubjubExtend::ADDITIVE_GENERATOR * r).as_bytes();
        let S = r + hash_to_scalar(&R, m);
        Signature::new(R, S.to_bytes())
    }
}

fn hash_to_scalar(a: &[u8], b: &[u8]) -> Fr {
    let ret = Params::new()
        .hash_length(64)
        .personal(b"FROST_RedJubjubM")
        .to_state()
        .update(a)
        .update(b)
        .finalize()
        .as_ref();
    Fr::from_hash(ret)
}
