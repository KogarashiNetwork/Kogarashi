mod rpc;
mod utils;
mod wallet;

use clap::{Parser, Subcommand};
use rpc::{get_balance, transfer};
use sp_keyring::RedjubjubKeyring as AccountKeyring;
use sp_runtime::AccountId32;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use utils::{extract_wallet, wallet_info};
use wallet::Wallet;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// display account list
    List,
    /// init redjubjub wallet
    Init,
    /// get balance
    Balance { address: Option<String> },
    /// fund to account
    Fund,
    /// transfer
    Transfer { to: Option<AccountId32> },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::List) => {
            println!("Alice: {:?}", AccountKeyring::Alice.to_account_id());
            println!("Bob: {:?}", AccountKeyring::Bob.to_account_id());
            println!("Charlie: {:?}", AccountKeyring::Charlie.to_account_id());
            println!("Dave: {:?}", AccountKeyring::Dave.to_account_id());
            println!("Eve: {:?}", AccountKeyring::Eve.to_account_id());
            println!("Ferdie: {:?}", AccountKeyring::Ferdie.to_account_id());
            println!("One: {:?}", AccountKeyring::One.to_account_id());
            println!("Two: {:?}", AccountKeyring::Two.to_account_id());
        }
        Some(Commands::Init) => {
            println!("Start Wallet Generation...");
            let wallet = Wallet::generate();
            let mut file = File::create("key.kog").expect("fail to create key file");
            file.write_all(&wallet.seed()).expect("fail to store key");
            wallet_info(&wallet);
        }
        Some(Commands::Balance { address }) => match address {
            Some(x) => {
                println!("Get {:?} Balance", x);
            }
            None => {
                let wallet = extract_wallet();
                let balance = get_balance(wallet.public()).await;
                println!("{:?} Balance", balance)
            }
        },
        Some(Commands::Fund) => {
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
        Some(Commands::Transfer { to }) => match to {
            Some(to) => {
                let wallet = extract_wallet();
                match transfer(wallet.pair(), to.clone(), 1000000000000).await {
                    Ok(tx_id) => {
                        println!("Transaction Success: {:?}", tx_id)
                    }
                    Err(err) => {
                        println!("Transaction Failure: {:?}", err)
                    }
                }
            }
            None => {
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
        },
        None => {}
    }
}
