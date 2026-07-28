use serde::Serialize;
use serde::de::DeserializeOwned;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::library::result::{ErrorModel, NBResult};

pub(super) const HEADER_LEN: usize = 8;

pub fn to_bytes<T: Serialize>(value: &T) -> NBResult<Vec<u8>> {
    return serde_json::to_vec(value).map_err(|e| ErrorModel::cache(e.to_string()));
}

pub fn from_bytes<T: DeserializeOwned>(bytes: &[u8]) -> NBResult<T> {
    return serde_json::from_slice(bytes).map_err(|e| ErrorModel::cache(e.to_string()));
}

/// Runs filesystem work on tokio's blocking pool. One hop per cache call, rather
/// than the per-syscall hop `tokio::fs` would cost on a write plus rename.
pub(super) async fn offload<T, F>(work: F) -> NBResult<T>
where
    F: FnOnce() -> NBResult<T> + Send + 'static,
    T: Send + 'static,
{
    return match tokio::task::spawn_blocking(work).await {
        Ok(result) => result,
        Err(error) => Err(ErrorModel::cache(error.to_string())),
    };
}

/// FNV-1a, 128 bit
fn hash(value: &str) -> u128 {
    const BASIS: u128 = 0x6c62272e07bb014262b821756295c58d;
    const PRIME: u128 = 0x0000000001000000000000000000013b;

    let mut hash = BASIS;
    for byte in value.as_bytes() {
        hash ^= *byte as u128;
        hash = hash.wrapping_mul(PRIME);
    }
    return hash;
}

pub(super) fn file_name(key: &str) -> String {
    return format!("{:032x}", hash(key));
}

pub(super) fn read_expiry(header: &[u8]) -> i64 {
    let mut bytes = [0u8; HEADER_LEN];
    bytes.copy_from_slice(&header[..HEADER_LEN]);
    return i64::from_le_bytes(bytes);
}

pub(super) fn is_expired(expiry: i64) -> bool {
    return expiry != 0 && now_millis() > expiry;
}

pub(super) fn now_millis() -> i64 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_millis() as i64)
        .unwrap_or(0);
}
