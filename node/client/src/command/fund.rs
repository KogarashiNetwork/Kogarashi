pub(crate) fn fund_command() {
    let wallet = extract_wallet();
    match transfer(
        wallet.pair(),
        AccountKeyring::Alice.to_account_id(),
        1000000000000,
    )
    .await
    {
        Ok(tx_id) => {
            println!("Transaction Success: {:?}", tx_id)
        }
        Err(err) => {
            println!("Transaction Failure: {:?}", err)
        }
    }
}
