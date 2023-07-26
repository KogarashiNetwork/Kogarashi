#![cfg_attr(not(feature = "std"), no_std)]

mod pair;
mod public;
mod signature;

use codec::{Decode, Encode};
use sp_core::crypto::CryptoTypeId;

pub use pair::Pair;
pub use public::Public;
pub use signature::Signature;

// signing context
pub(crate) const SIGNING_CTX: &[u8] = b"substrate";

/// An identifier used to match public keys against redjubjub keys
pub const CRYPTO_ID: CryptoTypeId = CryptoTypeId(*b"redj");

/// A secret seed. It's not called a "secret key" because ring doesn't expose the secret keys
/// of the key pair (yeah, dumb); as such we're forced to remember the seed manually if we
/// will need it later (such as for HDKD).
#[cfg(feature = "full_crypto")]
type Seed = [u8; 32];

/// A localized signature also contains sender information.
#[cfg(feature = "std")]
#[derive(PartialEq, Eq, Clone, Debug, Encode, Decode)]
pub struct LocalizedSignature {
    /// The signer of the signature.
    pub signer: Public,
    /// The signature itself.
    pub signature: Signature,
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;
    use serde_json;
    use sp_core::crypto::{Derive, Ss58Codec, DEV_ADDRESS, DEV_PHRASE};
    use sp_core::{DeriveJunction, Pair as TraitPair};

    #[test]
    fn default_phrase_should_be_used() {
        assert_eq!(
            Pair::from_string("//Alice///password", None)
                .unwrap()
                .public(),
            Pair::from_string(&format!("{}//Alice", DEV_PHRASE), Some("password"))
                .unwrap()
                .public(),
        );
        assert_eq!(
            Pair::from_string(&format!("{}/Alice", DEV_PHRASE), None)
                .as_ref()
                .map(Pair::public),
            Pair::from_string("/Alice", None).as_ref().map(Pair::public)
        );
    }

    #[test]
    fn default_address_should_be_used() {
        assert_eq!(
            Public::from_string(&format!("{}/Alice", DEV_ADDRESS)),
            Public::from_string("/Alice")
        );
    }

    #[test]
    fn derive_hard_should_work() {
        let pair = Pair::from_seed(&hex!(
            "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
        ));
        let derive_1 = pair
            .derive(Some(DeriveJunction::hard(1)).into_iter(), None)
            .unwrap()
            .0;
        let derive_1b = pair
            .derive(Some(DeriveJunction::hard(1)).into_iter(), None)
            .unwrap()
            .0;
        let derive_2 = pair
            .derive(Some(DeriveJunction::hard(2)).into_iter(), None)
            .unwrap()
            .0;
        assert_eq!(derive_1.public(), derive_1b.public());
        assert_ne!(derive_1.public(), derive_2.public());
    }

    #[test]
    fn derive_hard_public_should_fail() {
        let pair = Pair::from_seed(&hex!(
            "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
        ));
        let path = Some(DeriveJunction::hard(1));
        assert!(pair.public().derive(path.into_iter()).is_none());
    }

    #[test]
    fn generated_pair_should_work() {
        let (pair, _) = Pair::generate();
        let public = pair.public();
        let message = b"Something important";
        let signature = pair.sign(&message[..]);
        assert!(Pair::verify(&signature, &message[..], &public));
    }

    #[test]
    fn messed_signature_should_not_work() {
        let (pair, _) = Pair::generate();
        let public = pair.public();
        let message = b"Signed payload";
        let Signature(mut bytes) = pair.sign(&message[..]);
        bytes[0] = !bytes[0];
        bytes[2] = !bytes[2];
        let signature = Signature(bytes);
        assert!(!Pair::verify(&signature, &message[..], &public));
    }

    #[test]
    fn messed_message_should_not_work() {
        let (pair, _) = Pair::generate();
        let public = pair.public();
        let message = b"Something important";
        let signature = pair.sign(&message[..]);
        assert!(!Pair::verify(
            &signature,
            &b"Something unimportant",
            &public
        ));
    }

    #[test]
    fn seeded_pair_should_work() {
        let pair = Pair::from_seed(b"12345678901234567890123456789012");
        let public = pair.public();
        assert_eq!(
            public,
            Public::from_raw(hex!(
                "30d8f86abcba34339bbdab3f697341515ff136ad4f5705f514898ca9aa6dcfd3"
            ))
        );
        let message = hex!("2f8c6129d816cf51c374bc7f08c3e63ed156cf78aefb4a6550d97b87997977ee00000000000000000200d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a4500000000000000");
        let signature = pair.sign(&message[..]);
        assert!(Pair::verify(&signature, &message[..], &public));
    }

    #[test]
    fn ss58check_roundtrip_works() {
        let (pair, _) = Pair::generate();
        let public = pair.public();
        let s = public.to_ss58check();
        let cmp = Public::from_ss58check(&s).unwrap();
        assert_eq!(cmp, public);
    }

    #[test]
    fn signature_serialization_works() {
        let pair = Pair::from_seed(b"12345678901234567890123456789012");
        let message = b"Something important";
        let signature = pair.sign(&message[..]);
        let serialized_signature = serde_json::to_string(&signature).unwrap();
        // Signature is 64 bytes, so 128 chars + 2 quote chars
        assert_eq!(serialized_signature.len(), 130);
        let signature = serde_json::from_str(&serialized_signature).unwrap();
        assert!(Pair::verify(&signature, &message[..], &pair.public()));
    }

    #[test]
    fn signature_serialization_doesnt_panic() {
        fn deserialize_signature(text: &str) -> Result<Signature, serde_json::error::Error> {
            Ok(serde_json::from_str(text)?)
        }
        assert!(deserialize_signature("Not valid json.").is_err());
        assert!(deserialize_signature("\"Not an actual signature.\"").is_err());
        // Poorly-sized
        assert!(deserialize_signature("\"abc123\"").is_err());
    }
}
