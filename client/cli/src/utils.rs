use substrate_rpc::Wallet;

pub(crate) fn wallet_info(wallet: &Wallet) {
    println!("SS58 Address: {:?}", wallet.public().to_string());
    println!("Wallet ID: {:?}", wallet.to_account_id());
    println!("Wallet Seed: {:?}", wallet.seed());
}
