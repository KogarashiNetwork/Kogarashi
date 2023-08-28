use rand_core::RngCore;
use red_jubjub::{PublicKey, SecretKey, Signature};
use zkstd::common::{FftField, SigUtils};

use crate::{
    db::Db,
    merkle_tree::{MerkleProof, SparseMerkleTree},
    poseidon::FieldHasher,
    proof::Proof,
};

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub(crate) struct Transaction(Signature, TransactionData);
#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub(crate) struct TransactionData {
    sender_address: PublicKey,
    receiver_address: PublicKey,
    amount: u64,
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

#[derive(Debug, Default, Copy, Clone, PartialEq)]
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
            index: u64::from_le_bytes(index),
            balance: u64::from_le_bytes(balance),
            address: PublicKey::from_bytes(address).unwrap(),
            nonce: u64::from_le_bytes(nonce),
        })
    }

    fn to_bytes(self) -> [u8; Self::LENGTH] {
        let mut bytes = [0u8; 56];
        bytes[0..8].copy_from_slice(&self.index.to_le_bytes());
        bytes[8..16].copy_from_slice(&self.balance.to_le_bytes());
        bytes[16..48].copy_from_slice(&self.address.to_bytes());
        bytes[48..].copy_from_slice(&self.nonce.to_le_bytes());
        bytes
    }
}

impl UserData {
    pub fn new(index: u64, balance: u64, address: PublicKey) -> Self {
        Self {
            index,
            balance,
            address,
            ..Default::default()
        }
    }

    pub fn balance(&self) -> u64 {
        self.balance
    }

    pub fn address(&self) -> PublicKey {
        self.address
    }

    pub fn to_field_element<F: FftField>(self) -> F {
        let mut field = [0_u8; 64];
        field[0..56].copy_from_slice(&self.to_bytes()[0..56]);
        F::from_bytes_wide(&field)
    }
}

impl Transaction {
    pub fn to_field_element<F: FftField>(self) -> F {
        let mut field = [0_u8; 64];
        field.copy_from_slice(&self.to_bytes()[0..64]);
        F::from_bytes_wide(&field)
    }
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

pub(crate) struct Batch<F: FftField> {
    prev_root: F,
    new_root: F,
    transactions: Vec<Transaction>,
}

impl<F: FftField> Batch<F> {
    pub fn transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }

    pub fn roots(&self) -> (F, F) {
        (self.prev_root, self.new_root)
    }
}

#[derive(Default)]
pub(crate) struct RollupOperator<F: FftField, H: FieldHasher<F, 2>, const N: usize> {
    state_merkle: SparseMerkleTree<F, H, N>,
    db: Db,
    transactions: Vec<(Transaction, F)>,
    index_counter: u64,
}

impl<F: FftField, H: FieldHasher<F, 2>, const N: usize> RollupOperator<F, H, N> {
    const BATCH_SIZE: usize = 2;

    pub fn new(hasher: &H) -> Self {
        Self {
            state_merkle: SparseMerkleTree::new_empty(hasher, &[0; 64])
                .expect("Failed to create state merkle tree"),
            ..Default::default()
        }
    }

    pub fn execute_transaction(
        &mut self,
        transaction: Transaction,
        hasher: &H,
    ) -> Option<Batch<F>> {
        let Transaction(signature, transaction_data) = transaction;
        let sender = self.db.get_mut(&transaction_data.sender_address);

        let sender_index = sender.index;

        self.state_merkle
            .generate_membership_proof(sender_index)
            .check_membership(
                &self.state_merkle.root(),
                &sender.to_field_element(),
                hasher,
            )
            .expect("Sender is not presented in the state");

        assert!(transaction_data
            .sender_address
            .validate(&transaction_data.to_bytes(), signature));

        assert!(sender.balance >= transaction_data.amount);

        assert!(sender.balance >= transaction_data.amount);
        sender.balance -= transaction_data.amount;
        self.state_merkle
            .update(sender_index, sender.to_field_element(), hasher)
            .expect("Failed to update balance");

        let intermediate_root = self.state_merkle.root();

        let receiver = self.db.get_mut(&transaction_data.receiver_address);

        let receiver_index = receiver.index;

        self.state_merkle
            .generate_membership_proof(receiver_index)
            .check_membership(
                &self.state_merkle.root(),
                &receiver.to_field_element(),
                hasher,
            )
            .expect("Sender is not presented in the state");

        receiver.balance += transaction_data.amount;

        self.state_merkle
            .update(receiver_index, receiver.to_field_element(), hasher)
            .expect("Failed to update balance");

        self.transactions
            .push((transaction, self.state_merkle.root()));

        if self.transactions.len() >= Self::BATCH_SIZE {
            Some(self.create_batch())
            // create merkle tree
            //self.create_proof();
            // send proof to Verifier contract
        } else {
            None
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

    pub fn state_root(&self) -> F {
        self.state_merkle.root()
    }

    pub(crate) fn process_deposits(&mut self, txs: Vec<Transaction>, hasher: &H) {
        for t in txs {
            let user = UserData::new(self.index_counter, t.1.amount, t.1.sender_address);
            self.db.insert(user.address, user);
            self.index_counter += 1;

            self.state_merkle
                .update(user.index, user.to_field_element(), hasher)
                .expect("Failed to update user info");

            // Need to add deposits to the transactions vec as well
            // skipped just for easier test implementation
            // self.transactions.push((t, self.state_root()));
        }
    }
}
