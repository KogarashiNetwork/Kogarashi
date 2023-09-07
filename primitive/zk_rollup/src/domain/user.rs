use super::{FftField, PublicKey, SigUtils};
use zkstd::common::{Decode, Encode};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Encode, Decode)]
pub(crate) struct UserData {
    pub(crate) index: u64,
    pub(crate) balance: u64,
    pub(crate) address: PublicKey,
    pub(crate) nonce: u64,
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
