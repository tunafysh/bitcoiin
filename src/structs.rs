// src/structs.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ServerBlock {
    pub data: Vec<u8>,
    pub target: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct ClientBlock {
    pub data: Vec<u8>,
    pub target: Vec<u8>,
    pub nonce: u64,
}
