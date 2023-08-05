mod command;
mod rpc;
mod utils;
mod wallet;

use clap::{Parser, Subcommand};
use command::{balance_command, fund_command, init_command, list_command, transfer_command};
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
    Balance { person: Option<String> },
    /// fund to account
    Fund,
    /// transfer
    Transfer { person: Option<String> },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::List) => list_command(),
        Some(Commands::Init) => init_command(),
        Some(Commands::Balance { person }) => balance_command(person),
        Some(Commands::Fund) => fund_command(),
        Some(Commands::Transfer { person }) => transfer_command(person),
        None => {}
    }
}
