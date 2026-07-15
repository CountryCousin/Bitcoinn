use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BlockchainInfo {
    pub chain: String,
    pub blocks: u64,
    pub headers: u64,
    pub difficulty: f64,

    #[serde(rename = "verificationprogress")]
    pub verification_progress: f64,
}

#[derive(Debug, Deserialize)]
pub struct WalletInfo {
    #[serde(rename="walletname")]
    pub wallet_name: String,

    pub balance: f64,

    #[serde(rename="unconfirmed_balance")]
    pub unconfirmed_balance: f64,

    #[serde(rename="txcount")]
    pub tx_count: u64,
}