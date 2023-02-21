trait Dsa {
    type Hash;
    type Signature;
    type PublicKey;
    type PrivateKey;

    fn sign(self, msg: &[u8]) -> Self::Signature;

    fn verify(msg: &[u8], signature: Self::Signature, public_key: Self::PublicKey) -> bool;
}
