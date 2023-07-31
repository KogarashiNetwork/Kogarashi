use rpc::rpc_to_localhost;
use sp_keyring::RedjubjubKeyring as AccountKeyring;

mod extrinsic;
mod rpc;
mod utils;

#[tokio::main]
async fn main() {
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
