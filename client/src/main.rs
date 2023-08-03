use rpc::{get_storage, get_system_account_info, rpc_to_localhost};
use sp_keyring::RedjubjubKeyring as AccountKeyring;

mod extrinsic;
mod rpc;
mod utils;

#[tokio::main]
async fn main() {
    get_system_account_info(AccountKeyring::Alice.public()).await;
    get_storage().await;
    let xt = extrinsic::create_balance_transfer_xt(
        AccountKeyring::Alice,
        AccountKeyring::Bob.to_account_id(),
        1000,
    )
    .await;
    println!("Extrinsic: {xt}");
    match rpc_to_localhost("author_submitExtrinsic", [xt]).await {
        Ok(res) => println!("Result: {res}"),
        Err(err) => println!("Error: {err}"),
    }
}
