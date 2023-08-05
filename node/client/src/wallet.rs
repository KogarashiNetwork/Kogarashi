use sp_core::redjubjub::{Pair, Public};
use sp_core::Pair as TPair;
use sp_runtime::AccountId32;

#[derive(Clone)]
pub struct Wallet {
    pair: Pair,
    seed: [u8; 32],
}

impl Wallet {
    pub fn generate() -> Self {
        let (pair, seed) = Pair::from_entropy(b"abcdefghijklmnopqrstuvwx", None);
        Self { pair, seed }
    }

    pub fn pair(&self) -> Pair {
        self.pair.clone()
    }

    pub fn public(&self) -> Public {
        self.pair.public()
    }

    pub fn seed(&self) -> [u8; 32] {
        self.seed
    }

    pub fn from_seed(seed: [u8; 32]) -> Self {
        let pair = Pair::from_seed(&seed);
        Self { pair, seed }
    }

    fn to_raw_public(&self) -> [u8; 32] {
        *self.public().as_array_ref()
    }

    pub fn to_account_id(&self) -> AccountId32 {
        self.to_raw_public().into()
    }
}
