mod extrinsic;

use crate::utils::{black2_128concat, encoded_key};
use frame_system::AccountInfo;
use hex::FromHex;
use pallet_balances::AccountData;
use serde_json::{json, Value};
use sp_core::Decode;
use sp_core::{
    redjubjub::{Pair, Public},
    H256,
};
use sp_runtime::AccountId32;
use sp_version::RuntimeVersion;
use std::str::FromStr;

type AccountMeta = AccountInfo<u32, AccountData<u128>>;

const LOCALHOST_RPC_URL: &str = "http://localhost:9933";

pub async fn get_nonce(account: &AccountId32) -> u32 {
    let nonce_json = rpc_to_localhost("system_accountNextIndex", (account,))
        .await
        .unwrap();
    serde_json::from_value(nonce_json).unwrap()
}

pub async fn get_balance(account: Public) -> u128 {
    let account_info = get_system_account_info(account).await;
    account_info.data.free
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

pub async fn transfer(from: Pair, to: AccountId32, amout: u128) -> anyhow::Result<Value> {
    let encoded_tx = extrinsic::create_balance_transfer_xt(from, to, amout).await;
    rpc_to_localhost("author_submitExtrinsic", [encoded_tx]).await
}

async fn get_system_account_info(account: Public) -> AccountMeta {
    let prefix = encoded_key(b"System", b"Account");
    let postfix = black2_128concat(account);
    let res = rpc_to_localhost("state_getStorage", (format!("0x{}{}", prefix, postfix),))
        .await
        .unwrap();
    match res.as_str() {
        Some(raw_text) => {
            let data = Vec::from_hex(raw_text.replace("0x", "")).unwrap();
            AccountInfo::decode(&mut data.as_slice()).unwrap()
        }
        None => AccountInfo::default(),
    }
}

async fn rpc_to_localhost<Params: serde::Serialize>(
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wallet::Wallet;
    use sp_keyring::RedjubjubKeyring as AccountKeyring;
    use std::{thread, time::Duration};

    #[tokio::main]
    #[test]
    async fn rpc_test() {
        // set param
        let transfer_amount = 1000000000000;

        // generate wallet
        let zane = Wallet::generate();
        let before_balance = get_balance(zane.public()).await;

        // transfer
        transfer(
            AccountKeyring::Alice.pair(),
            zane.to_account_id(),
            transfer_amount,
        )
        .await
        .unwrap();

        // wait for inclusion
        thread::sleep(Duration::from_millis(15000));

        // check state transition
        let after_balance = get_balance(zane.public()).await;
        assert_eq!(before_balance + transfer_amount, after_balance)
    }
}
