use sp_core::redjubjub::{Pair, Public};
use sp_core::Pair as TPair;
use sp_runtime::AccountId32;

pub struct Wallet {
    pair: Pair,
    seed: [u8; 32],
}

impl Wallet {
    pub(crate) fn generate() -> Self {
        let (pair, seed) = Pair::from_entropy(b"abcdefghijklmnopqrstuvwx", None);
        Self { pair, seed }
    }

    pub(crate) fn public(&self) -> Public {
        self.pair.public()
    }

    fn to_raw_public(&self) -> [u8; 32] {
        *self.public().as_array_ref()
    }

    pub(crate) fn to_account_id(&self) -> AccountId32 {
        self.to_raw_public().into()
    }
}
