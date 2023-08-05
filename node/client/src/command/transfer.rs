use crate::rpc::transfer;
use crate::wallet::{extract_wallet, is_wallet_init};

use sp_keyring::RedjubjubKeyring as AccountKeyring;

pub(crate) async fn transfer_command(person: &str, amount: u128) {
    if is_wallet_init() {
        let wallet = extract_wallet();
        let to = match person {
            "Alice" => AccountKeyring::Alice.to_account_id(),
            "Bob" => AccountKeyring::Bob.to_account_id(),
            "Charlie" => AccountKeyring::Charlie.to_account_id(),
            "Dave" => AccountKeyring::Dave.to_account_id(),
            "Eve" => AccountKeyring::Eve.to_account_id(),
            "Ferdie" => AccountKeyring::Ferdie.to_account_id(),
            "One" => AccountKeyring::One.to_account_id(),
            "Two" => AccountKeyring::Two.to_account_id(),
            _ => AccountKeyring::Alice.to_account_id(),
        };
        match transfer(wallet.pair(), to.clone(), amount).await {
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
