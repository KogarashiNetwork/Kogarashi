use crate::rpc::transfer;
use crate::wallet::extract_wallet;

use sp_keyring::RedjubjubKeyring as AccountKeyring;

pub(crate) async fn transfer_command(person: &Option<String>) {
    let wallet = extract_wallet();
    let to = match person {
        Some(name) => match &name as &str {
            "Alice" => AccountKeyring::Alice.to_account_id(),
            "Bob" => AccountKeyring::Bob.to_account_id(),
            "Charlie" => AccountKeyring::Charlie.to_account_id(),
            "Dave" => AccountKeyring::Dave.to_account_id(),
            "Eve" => AccountKeyring::Eve.to_account_id(),
            "Ferdie" => AccountKeyring::Ferdie.to_account_id(),
            "One" => AccountKeyring::One.to_account_id(),
            "Two" => AccountKeyring::Two.to_account_id(),
            _ => AccountKeyring::Alice.to_account_id(),
        },
        None => AccountKeyring::Alice.to_account_id(),
    };
    match transfer(wallet.pair(), to.clone(), 1000000000000).await {
        Ok(tx_id) => {
            println!("Transaction Success: {:?}", tx_id)
        }
        Err(err) => {
            println!("Transaction Failure: {:?}", err)
        }
    }
}
