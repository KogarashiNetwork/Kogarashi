use crate::curve::JubjubExtended;
use crate::fp::Fp;

use zero_bls12_381::Fr;
use zero_crypto::behave::{CurveGroup, DigitalSig};

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
pub struct PublicKey(JubjubExtended);

impl DigitalSig for PublicKey {
    const LENGTH: usize = 32;
}

impl PublicKey {
    pub fn new(raw: JubjubExtended) -> Self {
        PublicKey(raw)
    }

    pub fn to_bytes(self) -> [u8; Self::LENGTH] {
        self.0.to_bytes()
    }

    pub fn validate(m: &[u8], sig: Signature) -> bool {
        let c = hash_to_scalar(&sig.s, m);
        let s = match Fr::from_bytes(sig.e) {
            Some(s) => s,
            None => return false,
        };
        todo!()
    }
}

#[derive(Clone)]
pub struct SecretKey(Fp);

impl SecretKey {
    pub fn sign(self, m: &[u8], mut rand: impl RngCore) -> Signature {
        let mut t = [0u8; 80];
        rand.fill_bytes(&mut t[..]);
        let r = hash_to_scalar(&t, m);
        let R = (JubjubExtended::ADDITIVE_GENERATOR * r).to_bytes();
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
        .finalize();
    Fr::from_hash(ret.as_ref())
}
