use std::collections::BTreeMap;

use red_jubjub::{PublicKey, Signature};
use zkstd::common::{FftField, SigUtils};

use crate::{
    merkle_tree::{MerkleProof, SparseMerkleTree},
    poseidon::FieldHasher,
    proof::Proof,
};

#[derive(Default, Clone, Copy)]
pub(crate) struct Transaction {
    sender_address: PublicKey,
    receiver_address: PublicKey,
    signature: Signature,
    amount: u64,
}

impl SigUtils<136> for Transaction {
    fn from_bytes(bytes: [u8; Self::LENGTH]) -> Option<Self> {
        let mut sender_address = [0_u8; 32];
        let mut receiver_address = [0_u8; 32];
        let mut signature = [0_u8; 64];
        let mut amount = [0_u8; 8];

        sender_address.copy_from_slice(&bytes[0..32]);
        receiver_address.copy_from_slice(&bytes[32..64]);
        signature.copy_from_slice(&bytes[64..128]);
        amount.copy_from_slice(&bytes[128..]);
        Some(Self {
            sender_address: PublicKey::from_bytes(sender_address).unwrap(),
            receiver_address: PublicKey::from_bytes(receiver_address).unwrap(),
            signature: Signature::from_bytes(signature).unwrap(),
            amount: u64::from_be_bytes(amount),
        })
    }

    fn to_bytes(self) -> [u8; Self::LENGTH] {
        let mut bytes = [0u8; 136];
        bytes[0..32].copy_from_slice(&self.sender_address.to_bytes());
        bytes[32..64].copy_from_slice(&self.receiver_address.to_bytes());
        bytes[64..128].copy_from_slice(&self.signature.to_bytes());
        bytes[128..].copy_from_slice(&self.amount.to_be_bytes());
        bytes
    }
}

#[derive(Default, Copy, Clone)]
pub(crate) struct UserData {
    index: u64,
    balance: u64,
    address: PublicKey,
    nonce: u64,
}

impl SigUtils<56> for UserData {
    fn from_bytes(bytes: [u8; Self::LENGTH]) -> Option<Self> {
        let mut index = [0_u8; 8];
        let mut balance = [0_u8; 8];
        let mut address = [0_u8; 32];
        let mut nonce = [0_u8; 8];

        index.copy_from_slice(&bytes[0..8]);
        balance.copy_from_slice(&bytes[8..16]);
        address.copy_from_slice(&bytes[16..48]);
        nonce.copy_from_slice(&bytes[48..]);
        Some(Self {
            index: u64::from_be_bytes(index),
            balance: u64::from_be_bytes(balance),
            address: PublicKey::from_bytes(address).unwrap(),
            nonce: u64::from_be_bytes(nonce),
        })
    }

    fn to_bytes(self) -> [u8; Self::LENGTH] {
        let mut bytes = [0u8; 56];
        bytes[0..8].copy_from_slice(&self.index.to_be_bytes());
        bytes[8..16].copy_from_slice(&self.balance.to_be_bytes());
        bytes[16..48].copy_from_slice(&self.address.to_bytes());
        bytes[48..].copy_from_slice(&self.nonce.to_be_bytes());
        bytes
    }
}

impl UserData {
    pub fn balance(&self) -> u64 {
        self.balance
    }

    pub fn to_field_element<F: FftField>(self) -> F {
        let mut field = [0_u8; 64];
        field.copy_from_slice(&self.to_bytes()[0..56]);
        F::from_bytes_wide(&field)
    }
}

impl Transaction {
    pub fn new(amount: u64) -> Self {
        Self {
            amount,
            ..Default::default()
        }
    }

    pub fn to_field_element<F: FftField>(self) -> F {
        let mut field = [0_u8; 64];
        field.copy_from_slice(&self.to_bytes()[0..64]);
        F::from_bytes_wide(&field)
    }
}
pub(crate) struct Batch<F: FftField> {
    prev_root: F,
    new_root: F,
    transactions: Vec<Transaction>,
}

#[derive(Default)]
pub(crate) struct RollupOperator<F: FftField, H: FieldHasher<F, 2>, const N: usize> {
    state_merkle: SparseMerkleTree<F, H, N>,
    users: BTreeMap<PublicKey, UserData>,
    transactions: Vec<(Transaction, F)>,
}

impl<F: FftField, H: FieldHasher<F, 2>, const N: usize> RollupOperator<F, H, N> {
    const BATCH_SIZE: usize = 25;
    pub fn execute_transaction(&mut self, transaction: Transaction, hasher: &H) {
        let sender = self
            .users
            .get_mut(&transaction.sender_address)
            .expect("Sender is not presented in the state");

        let sender_index = sender.index;

        let sender_hash = sender.to_field_element();

        self.state_merkle
            .generate_membership_proof(sender_index)
            .check_membership(&self.state_merkle.root(), &sender_hash, hasher)
            .expect("Sender is not presented in the state");

        transaction
            .sender_address
            .validate(&transaction.to_bytes(), transaction.signature);

        assert!(sender.balance >= transaction.amount);
        sender.balance -= transaction.amount;
        let sender_hash = sender.to_field_element();

        self.state_merkle
            .update(sender_index, sender_hash, hasher)
            .expect("Failed to update balance");

        let intermediate_root = self.state_merkle.root();

        let receiver = self
            .users
            .get_mut(&transaction.receiver_address)
            .expect("Sender is not presented in the state");

        let receiver_index = receiver.index;
        let receiver_hash = receiver.to_field_element();

        self.state_merkle
            .generate_membership_proof(receiver_index)
            .check_membership(&self.state_merkle.root(), &receiver_hash, hasher)
            .expect("Sender is not presented in the state");

        receiver.balance += transaction.amount;
        let receiver_hash = receiver.to_field_element();

        self.state_merkle
            .update(receiver_index, receiver_hash, hasher)
            .expect("Failed to update balance");

        self.transactions
            .push((transaction, self.state_merkle.root()));

        if self.transactions.len() > Self::BATCH_SIZE {
            self.create_batch();
            // create merkle tree
            //self.create_proof();
            // send proof to Verifier contract
        }
    }
    pub fn create_batch(&mut self) -> Batch<F> {
        let batch = Batch {
            prev_root: self.transactions[0].1,
            new_root: self.transactions[Self::BATCH_SIZE - 1].1,
            transactions: self.transactions[..Self::BATCH_SIZE]
                .iter()
                .map(|(t, _)| *t)
                .collect(),
        };
        self.transactions.drain(..Self::BATCH_SIZE);
        batch
    }

    pub fn create_proof(
        &self,
        batch_tree: SparseMerkleTree<F, H, N>,
        t_merkle_proofs: Vec<MerkleProof<F>>,
        sender_receiver_in_state_merkle_proofs: Vec<MerkleProof<F>>,
        state_roots: Vec<F>,
    ) -> Proof {
        Proof {}
    }
}
