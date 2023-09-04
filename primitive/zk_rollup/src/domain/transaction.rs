use crate::{merkle_tree::MerkleProof, poseidon::FieldHasher};

use super::{FftField, PublicKey, RngCore, SecretKey, SigUtils, Signature, UserData};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RollupTransactionInfo<F: FftField, H: FieldHasher<F, 2>, const N: usize> {
    pub(crate) transaction: Transaction,
    pub(crate) pre_root: F,
    pub(crate) post_root: F,
    pub(crate) pre_sender: UserData,
    pub(crate) pre_receiver: UserData,
    pub(crate) pre_sender_proof: MerkleProof<F, H, N>,
    pub(crate) pre_receiver_proof: MerkleProof<F, H, N>,
    pub(crate) post_sender_proof: MerkleProof<F, H, N>,
    pub(crate) post_receiver_proof: MerkleProof<F, H, N>,
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub(crate) struct Transaction(pub(crate) Signature, pub(crate) TransactionData);

impl Transaction {
    pub fn to_field_element<F: FftField>(self) -> F {
        let mut field = [0_u8; 64];
        field.copy_from_slice(&self.to_bytes()[0..64]);
        F::from_bytes_wide(&field)
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub(crate) struct TransactionData {
    pub(crate) sender_address: PublicKey,
    pub(crate) receiver_address: PublicKey,
    pub(crate) amount: u64,
}

impl TransactionData {
    pub fn new(sender_address: PublicKey, receiver_address: PublicKey, amount: u64) -> Self {
        Self {
            sender_address,
            receiver_address,
            amount,
        }
    }

    pub fn signed(self, secret_key: SecretKey, rand: impl RngCore) -> Transaction {
        let sig = secret_key.sign(&self.to_bytes(), rand);
        Transaction(sig, self)
    }
}

impl SigUtils<136> for Transaction {
    fn from_bytes(bytes: [u8; Self::LENGTH]) -> Option<Self> {
        let mut signature = [0_u8; 64];
        let mut transaction_data = [0_u8; 72];

        signature.copy_from_slice(&bytes[0..64]);
        transaction_data.copy_from_slice(&bytes[64..]);
        Some(Self(
            Signature::from_bytes(signature).unwrap(),
            TransactionData::from_bytes(transaction_data).unwrap(),
        ))
    }

    fn to_bytes(self) -> [u8; Self::LENGTH] {
        let mut bytes = [0u8; 136];
        bytes[0..64].copy_from_slice(&self.0.to_bytes());
        bytes[64..].copy_from_slice(&self.1.to_bytes());
        bytes
    }
}

impl SigUtils<72> for TransactionData {
    fn from_bytes(bytes: [u8; Self::LENGTH]) -> Option<Self> {
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

    fn to_bytes(self) -> [u8; Self::LENGTH] {
        let mut bytes = [0u8; 72];
        bytes[0..32].copy_from_slice(&self.sender_address.to_bytes());
        bytes[32..64].copy_from_slice(&self.receiver_address.to_bytes());
        bytes[64..].copy_from_slice(&self.amount.to_le_bytes());
        bytes
    }
}
