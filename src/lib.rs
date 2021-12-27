#[cfg(test)]
mod sign_test {
    use ed25519::{Error, Signature};
    use ed25519_dalek::{Keypair, PublicKey, Signer, Verifier};
    use rand_core::OsRng;

    pub struct HelloSigner<S>
    where
        S: Signer<Signature>,
    {
        pub signing_key: S,
    }

    impl<S> HelloSigner<S>
    where
        S: Signer<Signature>,
    {
        pub fn sign(&self, person: &str) -> Signature {
            // NOTE: use `try_sign` if you'd like to be able to handle
            // errors from external signing services/devices (e.g. HSM/KMS)
            // <https://docs.rs/signature/latest/signature/trait.Signer.html#tymethod.try_sign>
            self.signing_key.sign(format_message(person).as_bytes())
        }
    }

    pub struct HelloVerifier<V> {
        pub verify_key: V,
    }

    impl<V> HelloVerifier<V>
    where
        V: Verifier<Signature>,
    {
        pub fn verify(&self, person: &str, signature: &Signature) -> Result<(), Error> {
            self.verify_key
                .verify(format_message(person).as_bytes(), signature)
        }
    }

    fn format_message(person: &str) -> String {
        format!("Hello, {}!", person)
    }

    /// `HelloSigner` defined above instantiated with `ed25519-dalek` as
    /// the signing provider.
    pub type DalekHelloSigner = HelloSigner<Keypair>;

    /// `HelloVerifier` defined above instantiated with `ed25519-dalek`
    /// as the signature verification provider.
    pub type DalekHelloVerifier = HelloVerifier<PublicKey>;

    #[test]
    fn encrypt_and_decrypt() {
        let signing_key = Keypair::generate(&mut OsRng);
        let signer = DalekHelloSigner { signing_key };
        let person = "transaction message"; // Message to sign
        let signature = signer.sign(person);

        let verify_key: PublicKey = signer.signing_key.public;
        let verifier = DalekHelloVerifier { verify_key };
        assert!(verifier.verify(person, &signature).is_ok());
    }
}
