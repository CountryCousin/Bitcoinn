use thiserror::Error;

#[derive(Error, Debug)]
pub enum RpcError {

    #[error("Connection failed")]
    Connection,

    #[error("Invalid credentials")]
    Authentication,

    #[error("RPC Error: {0}")]
    Rpc(String),

    #[error("Serialization Error")]
    Parse,
}