use std::collections::BTreeMap;

use red_jubjub::{PublicKey, Signature};
use zkstd::common::{FftField, SigUtils};

use crate::{
    merkle_tree::{MerkleProof, SparseMerkleTree},
    poseidon::FieldHasher,
    proof::Proof,
};

#[derive(Default, Clone)]
pub(crate) struct Transaction {
    sender_address: PublicKey,
    receiver_address: PublicKey,
    signature: Signature,
    amount: u64,
}

impl SigUtils<136> for Transaction {
    fn from_bytes(bytes: [u8; Self::LENGTH]) -> Option<Self> {
        Some(Self {
            sender_address: PublicKey::from_bytes(&bytes[0..32]),
            receiver_address: PublicKey::from_bytes(&bytes[32..64]),
            signature: Signature::from_bytes(&bytes[64..128]),
            amount: u64::from_be_bytes(bytes[128..]),
        })
    }

    fn to_bytes(self) -> [u8; Self::LENGTH] {
        let mut bytes = [0u8; 136];
        bytes[0..32].copy_from_slice(&self.sender_address);
        bytes[32..64].copy_from_slice(&self.receiver_address);
        bytes[64..128].copy_from_slice(&self.signature);
        bytes[128..].copy_from_slice(&self.amount.to_be_bytes());
    }
}

#[derive(Default)]
pub(crate) struct UserData {
    index: u64,
    balance: u64,
    address: PublicKey,
    nonce: u64,
}

impl SigUtils<64> for UserData {
    fn from_bytes(bytes: [u8; Self::LENGTH]) -> Option<Self> {
        Some(Self {
            index: u64::from_be_bytes(&bytes[0..8]),
            balance: u64::from_be_bytes(&bytes[8..16]),
            address: PublicKey::from_bytes(&bytes[16..48]),
            nonce: u64::from_be_bytes(bytes[48..]),
        })
    }

    fn to_bytes(self) -> [u8; Self::LENGTH] {
        let mut bytes = [0u8; 136];
        bytes[0..8].copy_from_slice(&self.index.to_be_bytes());
        bytes[8..16].copy_from_slice(&self.balance.to_be_bytes());
        bytes[16..48].copy_from_slice(&self.address);
        bytes[48..].copy_from_slice(&self.nonce.to_be_bytes());
    }
}

impl UserData {
    pub fn balance(&self) -> u64 {
        self.balance
    }
}

impl Transaction {
    pub fn new(amount: u64) -> Self {
        Self {
            amount,
            ..Default::default()
        }
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
            .get_mut(&&transaction.sender_address)
            .expect("Sender is not presented in the state");

        let sender_index = sender.index;

        let sender_hash = F::zero(); // do hashing

        self.state_merkle
            .generate_membership_proof(sender_index)
            .check_membership(&self.state_merkle.root(), &sender_hash, hasher)
            .expect("Sender is not presented in the state");

        transaction
            .sender_address
            .validate(&transaction.to_bytes(), transaction.signature);

        assert!(sender.balance >= transaction.amount);
        sender.balance -= transaction.amount;
        let user_hash = F::zero(); // do hashing

        self.state_merkle
            .update(sender_index, user_hash, hasher)
            .expect("Failed to update balance");

        let intermediate_root = self.state_merkle.root();

        let receiver = self
            .users
            .get_mut(&&transaction.receiver_address)
            .expect("Sender is not presented in the state");

        let receiver_index = receiver.index;
        let receiver_hash = F::zero(); // do hashing

        self.state_merkle
            .generate_membership_proof(receiver_index)
            .check_membership(&self.state_merkle.root(), &receiver_hash, hasher)
            .expect("Sender is not presented in the state");

        receiver.balance += transaction.amount;
        let receiver_hash = F::zero(); // do hashing

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
                .map(|(t, _)| t.clone())
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
