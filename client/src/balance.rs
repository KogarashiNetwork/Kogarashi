mod rpc;
mod utils;
use std::env;
use sp_runtime::AccountId32;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let account = &args[1];
    println!("{account}");
    rpc::get_storage().await;
}
