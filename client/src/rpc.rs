use std::str::FromStr;

use serde_json::{json, Value};
use sp_core::{H256, Decode};
use sp_version::RuntimeVersion;
use sp_core::{hashing};
use sp_keyring::AccountKeyring;

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

pub async fn get_storage() {
    let storage_prefix = "System";
    let storage_name = "Account";

    let account = AccountKeyring::Alice.to_account_id();

    let storage_prefix_hashed = hashing::twox_128(storage_prefix.as_bytes());
    let storage_name_hashed = hashing::twox_128(storage_name.as_bytes());
    let account_id_hashed = hashing::blake2_128(account.as_ref());

    let mut storage_key = Vec::new();
    storage_key.extend_from_slice(&storage_prefix_hashed);
    storage_key.extend_from_slice(&storage_name_hashed);
    storage_key.extend_from_slice(&account_id_hashed);
    storage_key.extend_from_slice(account.as_ref());

    let storage_key_hex = format!("0x{}", hex::encode(&storage_key));

    let result_hex = rpc_to_localhost("state_getStorage", (storage_key_hex,))
        .await
        .unwrap();

    let result_scaled = hex::decode(result_hex.as_str().unwrap().trim_start_matches("0x")).unwrap();

    type PolkadotAccountInfo = frame_system::AccountInfo<u32, pallet_balances::AccountData<u128>>;
    let account_info = PolkadotAccountInfo::decode(&mut result_scaled.as_ref()).expect("Failed to decode account info");
    println!("{:?}", account_info.data.free);
}
