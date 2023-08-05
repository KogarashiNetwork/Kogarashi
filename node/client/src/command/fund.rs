use crate::rpc::transfer;
use crate::wallet::{extract_wallet, is_wallet_init};

use sp_keyring::RedjubjubKeyring as AccountKeyring;

pub(crate) async fn fund_command() {
    if is_wallet_init() {
        let wallet = extract_wallet();
        match transfer(
            wallet.pair(),
            AccountKeyring::Alice.to_account_id(),
            1000000000000,
        )
        .await
        {
            Ok(tx_id) => {
                println!("Transaction Success: {:?}", tx_id)
            }
            Err(err) => {
                println!("Transaction Failure: {:?}", err)
            }
        }
    } else {
        println!("Please Init Wallet...");
    }
}
