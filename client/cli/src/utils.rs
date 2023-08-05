use std::fs::File;
use std::io::Read;

use substrate_rpc::Wallet;

pub(crate) fn wallet_info(wallet: &Wallet) {
    println!("SS58 Address: {:?}", wallet.public().to_string());
    println!("Wallet ID: {:?}", wallet.to_account_id());
    println!("Wallet Seed: {:?}", wallet.seed());
}

pub(crate) fn extract_wallet() -> Wallet {
    let mut f = File::open("key.kog").unwrap();
    let mut secret = vec![];
    f.read_to_end(&mut secret).unwrap();
    let seed: [u8; 32] = secret[..32].try_into().unwrap();
    Wallet::from_seed(seed)
}
