use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::library::result::{ErrorModel, NBResult};

pub fn to_bytes<T: Serialize>(value: &T) -> NBResult<Vec<u8>> {
    return serde_json::to_vec(value).map_err(|e| ErrorModel::cache(e.to_string()));
}

pub fn from_bytes<T: DeserializeOwned>(bytes: &[u8]) -> NBResult<T> {
    return serde_json::from_slice(bytes).map_err(|e| ErrorModel::cache(e.to_string()));
}
