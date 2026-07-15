mod cli;
mod commands;
mod config;
mod error;
mod models;
mod rpc;

use clap::Parser;

use cli::{Cli, Commands};

use config::Config;

use rpc::RpcClient;

fn main() {

    let cli = Cli::parse();

    let config = Config::load();

    let client = RpcClient::new(config);

    match cli.command {

        Commands::BlockchainInfo => {

            commands::blockchain_info(&client);
        }

        Commands::WalletInfo => {

            commands::wallet_info(&client);
        }

        Commands::Balance => {

            commands::balance(&client);
        }

        Commands::NewAddress => {

            commands::new_address(&client);
        }

        Commands::Rpc { method, params } => {

            commands::generic_rpc(&client, method, params);
        }
    }
}