use crate::config::Config;
use reqwest::blocking::Client;
use serde_json::{json, Value};

pub struct RpcClient {
    client: Client,
    config: Config,
}

impl RpcClient {
    pub fn new(config: Config) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    pub fn call(
    &self,
    method: &str,
    params: Vec<serde_json::Value>,
) -> Result<serde_json::Value, String> {
    use serde_json::{json, Value};

    let body = json!({
        "jsonrpc": "1.0",
        "id": "rust-cli",
        "method": method,
        "params": params
    });

    let response = self
        .client
        .post(&self.config.rpc_url)
        .basic_auth(
            &self.config.rpc_user,
            Some(&self.config.rpc_password),
        )
        .json(&body)
        .send()
        .map_err(|e| format!("Request failed: {}", e))?;

    let status = response.status();
    let text = response
        .text()
        .map_err(|e| format!("Couldn't read response: {}", e))?;

    println!("HTTP Status: {}", status);
    println!("Raw response:\n{}", text);

    let value: Value = serde_json::from_str(&text)
        .map_err(|e| format!("JSON parse error: {}", e))?;

    if !value["error"].is_null() {
        return Err(value["error"].to_string());
    }

    Ok(value["result"].clone())
}
}