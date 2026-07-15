use crate::rpc::RpcClient;
use serde_json::Value;

pub fn blockchain_info(client: &RpcClient) {
    match client.call("getblockchaininfo", vec![]) {
        Ok(v) => println!("{}", serde_json::to_string_pretty(&v).unwrap()),
        Err(e) => eprintln!("{}", e),
    }
}

pub fn wallet_info(client: &RpcClient) {
    match client.call("getwalletinfo", vec![]) {
        Ok(v) => println!("{}", serde_json::to_string_pretty(&v).unwrap()),
        Err(e) => eprintln!("{}", e),
    }
}

pub fn balance(client: &RpcClient) {
    match client.call("getbalance", vec![]) {
        Ok(v) => println!("{}", v),
        Err(e) => eprintln!("{}", e),
    }
}

pub fn new_address(client: &RpcClient) {
    match client.call("getnewaddress", vec![]) {
        Ok(v) => println!("{}", v),
        Err(e) => eprintln!("{}", e),
    }
}

pub fn generic_rpc(
    client: &RpcClient,
    method: String,
    params: Vec<String>,
) {
    let json_params: Vec<Value> = params
        .into_iter()
        .map(|p| {
            if let Ok(i) = p.parse::<i64>() {
                Value::from(i)
            } else if let Ok(f) = p.parse::<f64>() {
                Value::from(f)
            } else if p == "true" {
                Value::Bool(true)
            } else if p == "false" {
                Value::Bool(false)
            } else {
                Value::String(p)
            }
        })
        .collect();

    match client.call(&method, json_params) {
        Ok(v) => println!("{}", serde_json::to_string_pretty(&v).unwrap()),
        Err(e) => eprintln!("{}", e),
    }
}