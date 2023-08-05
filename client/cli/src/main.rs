mod utils;

use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use substrate_rpc::{get_balance, transfer, AccountKeyring, Wallet};
use utils::wallet_info;

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
    /// init redjubjub wallet
    Init,
    /// get balance
    Balance { address: Option<String> },
    /// fund to account
    Fund,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
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
                let mut f = File::open("key.kog").unwrap();
                let mut secret = vec![];
                f.read_to_end(&mut secret).unwrap();
                let seed: [u8; 32] = secret[..32].try_into().unwrap();
                let wallet = Wallet::from_seed(seed);
                wallet_info(&wallet);
                let balance = get_balance(wallet.public()).await;
                println!("{:?} Balance", balance)
            }
        },
        Some(Commands::Fund) => {
            let mut f = File::open("key.kog").unwrap();
            let mut secret = vec![];
            f.read_to_end(&mut secret).unwrap();
            let seed: [u8; 32] = secret[..32].try_into().unwrap();
            let wallet = Wallet::from_seed(seed);
            match transfer(AccountKeyring::Alice, wallet.to_account_id(), 1000000000000).await {
                Ok(tx_id) => {
                    println!("Transaction Success: {:?}", tx_id)
                }
                Err(err) => {
                    println!("Transaction Failure: {:?}", err)
                }
            }
        }
        None => {}
    }
}
