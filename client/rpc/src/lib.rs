mod extrinsic;
mod rpc;
mod utils;
mod wallet;

pub use rpc::{get_balance, transfer};
pub use sp_keyring::RedjubjubKeyring as AccountKeyring;
use std::{thread, time::Duration};
pub use wallet::Wallet;

#[tokio::main]
async fn main() {
    // set param
    let transfer_amount = 1000000000000;

    // generate wallet
    let zane = Wallet::generate();
    let before_balance = get_balance(zane.public()).await;

    // transfer
    transfer(AccountKeyring::Alice, zane.to_account_id(), transfer_amount)
        .await
        .unwrap();

    // wait for inclusion
    thread::sleep(Duration::from_millis(5000));

    // check state transition
    let after_balance = get_balance(zane.public()).await;
    assert_eq!(before_balance + transfer_amount, after_balance)
}
