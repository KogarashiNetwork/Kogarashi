extern crate ed25519_dalek;
extern crate rand;

use ed25519_dalek::{Keypair, Signature, Signer};
use rand::rngs::OsRng;

#[cfg(test)]
mod minimum_test {
    use super::*;

    #[test]
    fn encrypt_and_decrypt() {
        let keypair: Keypair = Keypair::generate(&mut OsRng);
        let message: &[u8] = b"transaction message example";
        let signature: Signature = keypair.sign(message);
        assert!(keypair.verify(message, &signature).is_ok());
    }
}
