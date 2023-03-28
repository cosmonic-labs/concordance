use std::error::Error;

use serde::{Deserialize, Serialize};

pub mod commands;
pub mod events;

pub fn deserialize<'de, T: Deserialize<'de>>(
    buf: &'de [u8],
) -> Result<T, Box<dyn Error + Send + Sync>> {
    serde_json::from_slice(buf).map_err(|e| format!("Deserialization failure: {e:?}").into())
}

pub fn serialize<T: Serialize>(data: &T) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
    serde_json::to_vec(data).map_err(|e| format!("Serialization failure: {e:?}").into())
}
