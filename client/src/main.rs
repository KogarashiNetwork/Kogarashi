use sp_keyring::AccountKeyring;
use subxt::{ClientBuilder, DefaultConfig, PairSigner, PolkadotExtrinsicParams};

#[subxt::subxt(runtime_metadata_path = "./artifacts/polkadot_metadata.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let signer = PairSigner::new(AccountKeyring::Alice.pair());

    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>>();

    // Submit the `transfer` extrinsic from Alice's account to Bob's.
    let dest = AccountKeyring::Bob.to_account_id().into();

    // Obtain an extrinsic, calling the "transfer" function in
    // the "balances" pallet.
    let extrinsic = api.tx().balances().transfer(dest, 123_456_789_012_345)?;

    // Sign and submit the extrinsic, returning its hash.
    let tx_hash = extrinsic.sign_and_submit_default(&signer).await?;

    println!("Balance transfer extrinsic submitted: {}", tx_hash);

    Ok(())
}
