use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub rpc_url: String,
    pub rpc_user: String,
    pub rpc_password: String,
    pub wallet: String,
}

impl Config {
    pub fn load() -> Self {
        dotenv().ok();

        Self {
            rpc_url: env::var("RPC_URL").unwrap(),
            rpc_user: env::var("RPC_USER").unwrap(),
            rpc_password: env::var("RPC_PASSWORD").unwrap(),
            wallet: env::var("WALLET").unwrap_or("default".into()),
        }
    }
}