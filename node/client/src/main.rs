mod command;
mod rpc;
mod utils;
mod wallet;

use clap::{Parser, Subcommand};
use command::{balance_command, fund_command, init_command, list_command, transfer_command};
use std::path::PathBuf;

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
    Transfer { person: String, amount: u128 },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::List) => list_command(),
        Some(Commands::Init) => init_command(),
        Some(Commands::Balance { person }) => balance_command(person).await,
        Some(Commands::Fund) => fund_command().await,
        Some(Commands::Transfer { person, amount }) => transfer_command(person, *amount).await,
        None => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time::Duration};

    #[cfg(feature = "integration")]
    #[tokio::main]
    #[test]
    async fn cli_test() {
        // print account list
        println!("List Command");
        list_command();

        // init wallet
        println!("\n\nInit Command");
        init_command();

        // fund to wallet
        println!("\n\nFund Command");
        fund_command().await;
        thread::sleep(Duration::from_millis(15000));

        // check balance
        println!("\n\nBalance Command");
        balance_command(&None).await;

        // transfer to Alice
        println!("\n\nTransfer Command");
        transfer_command("Alice", 500).await;
        thread::sleep(Duration::from_millis(15000));

        // check state transition
        println!("\n\nBalance Command");
        balance_command(&None).await;
    }
}
