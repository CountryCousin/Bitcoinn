use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    BlockchainInfo,

    WalletInfo,

    Balance,

    NewAddress,

    Rpc {
        method: String,

        params: Vec<String>,
    },
}