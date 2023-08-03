use crate::utils::{black2_128concat, encoded_key};
use hex::encode;
use serde_json::{json, Value};
use sp_core::{redjubjub::Public, H256};
use sp_io::hashing::twox_128;
use sp_version::RuntimeVersion;
use std::str::FromStr;

const LOCALHOST_RPC_URL: &str = "http://localhost:9933";

pub async fn rpc_to_localhost<Params: serde::Serialize>(
    method: &str,
    params: Params,
) -> anyhow::Result<Value> {
    let client = reqwest::Client::new();
    let mut body: Value = client
        .post(LOCALHOST_RPC_URL)
        .json(&json! {{
            "id": 1,
            "jsonrpc": "2.0",
            "method": method,
            "params": params
        }})
        .send()
        .await?
        .json()
        .await?;

    Ok(body["result"].take())
}

pub async fn get_nonce(account: &sp_runtime::AccountId32) -> u32 {
    let nonce_json = rpc_to_localhost("system_accountNextIndex", (account,))
        .await
        .unwrap();
    serde_json::from_value(nonce_json).unwrap()
}

pub async fn get_genesis_hash() -> H256 {
    let genesis_hash_json = rpc_to_localhost("chain_getBlockHash", [0]).await.unwrap();
    let genesis_hash_hex = genesis_hash_json.as_str().unwrap();
    H256::from_str(genesis_hash_hex).unwrap()
}

pub async fn get_runtime_version() -> RuntimeVersion {
    let runtime_version_json = rpc_to_localhost("state_getRuntimeVersion", ())
        .await
        .unwrap();
    serde_json::from_value(runtime_version_json).unwrap()
}

pub async fn get_system_account_info(account: Public) {
    let prefix = encoded_key(b"System", b"Account");
    let postfix = black2_128concat(account);
    let account_info = rpc_to_localhost("state_getStorage", (format!("0x{}{}", prefix, postfix),))
        .await
        .unwrap();
    println!("{:?}", account_info);
}

pub async fn get_storage() {
    let module = twox_128(b"Sudo");
    let method = twox_128(b"Key");
    let storage_value = rpc_to_localhost(
        "state_getStorage",
        (format!("0x{}{}", encode(module), encode(&method)),),
    )
    .await
    .unwrap();
    println!("{:?}", storage_value);
}
