mod extrinsic;
mod rpc;
mod utils;
mod wallet;

use rpc::{get_balance, rpc_to_localhost};
use sp_keyring::RedjubjubKeyring as AccountKeyring;
use std::{thread, time::Duration};
use wallet::Wallet;

#[tokio::main]
async fn main() {
    // set param
    let transfer_amount = 1000000000000;

    // generate wallet
    let zane = Wallet::generate();
    let before_balance = get_balance(zane.public()).await;

    // send transaction
    let encoded_tx = extrinsic::create_balance_transfer_xt(
        AccountKeyring::Alice,
        zane.to_account_id(),
        transfer_amount,
    )
    .await;
    let tx = rpc_to_localhost("author_submitExtrinsic", [encoded_tx]).await;

    // wait block inclusion
    thread::sleep(Duration::from_millis(5000));

    match tx {
        Ok(_) => {
            // check state transition
            let after_balance = get_balance(zane.public()).await;
            assert_eq!(before_balance + transfer_amount, after_balance)
        }
        Err(_) => {
            assert!(false, "transaction failure")
        }
    }
}
