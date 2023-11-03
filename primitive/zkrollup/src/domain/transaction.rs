use crate::{merkle_tree::MerkleProof, poseidon::FieldHasher};

use super::{FftField, PublicKey, RngCore, SecretKey, SigUtils, Signature, UserData};
use zkstd::common::*;

#[derive(Clone, Debug, PartialEq, Eq, Default, Encode, Decode)]
pub(crate) struct RollupTransactionInfo<P: RedDSA, H: FieldHasher<P::Range, 2>, const N: usize> {
    pub(crate) transaction: Transaction<P>,
    pub(crate) pre_root: P::Range,
    pub(crate) post_root: P::Range,
    pub(crate) pre_sender: UserData<P>,
    pub(crate) pre_receiver: UserData<P>,
    pub(crate) pre_sender_proof: MerkleProof<P::Range, H, N>,
    pub(crate) pre_receiver_proof: MerkleProof<P::Range, H, N>,
    pub(crate) post_sender_proof: MerkleProof<P::Range, H, N>,
    pub(crate) post_receiver_proof: MerkleProof<P::Range, H, N>,
    pub(crate) is_withdrawal: bool,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Decode, Encode)]
pub struct Transaction<P: RedDSA>(pub(crate) Signature, pub(crate) TransactionData<P>);

impl<P: RedDSA> Transaction<P> {
    pub fn to_field_element(self) -> P::Range {
        let mut field = [0_u8; 64];
        field.copy_from_slice(&self.to_bytes()[0..64]);
        P::Range::from_bytes_wide(&field)
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Decode, Encode)]
pub struct TransactionData<P: RedDSA> {
    pub(crate) sender_address: PublicKey<P>,
    pub(crate) receiver_address: PublicKey<P>,
    pub(crate) amount: u64,
}

impl<P: RedDSA> TransactionData<P> {
    pub fn new(sender_address: PublicKey<P>, receiver_address: PublicKey<P>, amount: u64) -> Self {
        Self {
            sender_address,
            receiver_address,
            amount,
        }
    }

    pub fn signed(self, secret_key: SecretKey<P>, rand: impl RngCore) -> Transaction<P> {
        let sig = secret_key.sign(&self.to_bytes(), rand);
        Transaction(sig, self)
    }
}

impl<P: RedDSA> SigUtils<136> for Transaction<P> {
    fn from_bytes(bytes: [u8; 136]) -> Option<Self> {
        let mut signature = [0_u8; 64];
        let mut transaction_data = [0_u8; 72];

        signature.copy_from_slice(&bytes[0..64]);
        transaction_data.copy_from_slice(&bytes[64..]);
        Some(Self(
            Signature::from_bytes(signature).unwrap(),
            TransactionData::from_bytes(transaction_data).unwrap(),
        ))
    }

    fn to_bytes(self) -> [u8; 136] {
        let mut bytes = [0u8; 136];
        bytes[0..64].copy_from_slice(&self.0.to_bytes());
        bytes[64..].copy_from_slice(&self.1.to_bytes());
        bytes
    }
}

impl<P: RedDSA> SigUtils<72> for TransactionData<P> {
    fn from_bytes(bytes: [u8; 72]) -> Option<Self> {
        let mut sender_address = [0_u8; 32];
        let mut receiver_address = [0_u8; 32];
        let mut amount = [0_u8; 8];

        sender_address.copy_from_slice(&bytes[0..32]);
        receiver_address.copy_from_slice(&bytes[32..64]);
        amount.copy_from_slice(&bytes[64..]);
        Some(Self {
            sender_address: PublicKey::from_bytes(sender_address).unwrap(),
            receiver_address: PublicKey::from_bytes(receiver_address).unwrap(),
            amount: u64::from_le_bytes(amount),
        })
    }

    fn to_bytes(self) -> [u8; 72] {
        let mut bytes = [0u8; 72];
        bytes[0..32].copy_from_slice(&self.sender_address.to_bytes());
        bytes[32..64].copy_from_slice(&self.receiver_address.to_bytes());
        bytes[64..].copy_from_slice(&self.amount.to_le_bytes());
        bytes
    }
}
