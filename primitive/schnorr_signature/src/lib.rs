use sha2::{Digest, Sha512};
use zero_crypto::common::{Group, RngCore};
use zero_ed25519::{Ed25519Affine, Ed25519Extend, Fp};

pub struct KeyPair {
    private_key: Fp,
    public_key: Ed25519Affine,
}

impl KeyPair {
    fn new(rand: impl RngCore) -> Self {
        let private_key = Fp::random(rand);
        let public_key = Ed25519Affine::from(Ed25519Extend::ADDITIVE_GENERATOR * private_key);
        Self {
            private_key,
            public_key,
        }
    }

    fn sign(self, message: &[u8]) -> [u8; 64] {
        let mut hash_output: [u8; 64] = [0; 64];
        let mut hasher = Sha512::new();
        hasher.update(message);
        hasher.finalize().into()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(true);
    }
}
