use crate::wallet::{wallet_info, Wallet};

use std::fs::File;
use std::io::Write;

pub(crate) fn init_command() {
    println!("Start Wallet Generation...");
    let wallet = Wallet::generate();
    let mut file = File::create("key.kog").expect("fail to create key file");
    file.write_all(&wallet.seed()).expect("fail to store key");
    wallet_info(&wallet);
}
