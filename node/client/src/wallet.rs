use sp_core::redjubjub::{Pair, Public};
use sp_core::Pair as TPair;
use sp_runtime::AccountId32;
use std::fs::{self, File};
use std::io::Read;

const KEY_PATH: &str = "key.kog";

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

pub(crate) fn wallet_info(wallet: &Wallet) {
    println!("SS58 Address: {:?}", wallet.public().to_string());
    println!("Wallet ID: {:?}", wallet.to_account_id());
    println!("Wallet Seed: {:?}", wallet.seed());
}

pub(crate) fn extract_wallet() -> Wallet {
    let mut f = File::open(KEY_PATH).unwrap();
    let mut secret = vec![];
    f.read_to_end(&mut secret).unwrap();
    let seed: [u8; 32] = secret[..32].try_into().unwrap();
    Wallet::from_seed(seed)
}

pub(crate) fn is_wallet_init() -> bool {
    match fs::metadata(KEY_PATH) {
        Ok(_) => true,
        Err(_) => false,
    }
}
