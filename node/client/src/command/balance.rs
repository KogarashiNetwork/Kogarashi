use crate::rpc::get_balance;
use crate::wallet::extract_wallet;

use sp_keyring::RedjubjubKeyring as AccountKeyring;

pub(crate) async fn balance_command(person: &Option<String>) {
    let wallet = match person {
        Some(name) => match &name as &str {
            "Alice" => AccountKeyring::Alice.public(),
            "Bob" => AccountKeyring::Bob.public(),
            "Charlie" => AccountKeyring::Charlie.public(),
            "Dave" => AccountKeyring::Dave.public(),
            "Eve" => AccountKeyring::Eve.public(),
            "Ferdie" => AccountKeyring::Ferdie.public(),
            "One" => AccountKeyring::One.public(),
            "Two" => AccountKeyring::Two.public(),
            _ => extract_wallet().public(),
        },
        None => extract_wallet().public(),
    };
    let balance = get_balance(wallet).await;
    println!("{:?} Balance", balance)
}
