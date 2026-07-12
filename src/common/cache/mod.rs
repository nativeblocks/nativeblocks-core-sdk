use serde::de::DeserializeOwned;

use crate::common::json;
use crate::common::result::{NBError, NBResult};

#[uniffi::export(with_foreign)]
pub trait CacheProvider: Send + Sync {
    fn save_bytes(&self, key: String, value: Vec<u8>, ttl_millis: Option<i64>) -> Result<(), NBError>;
    fn get_bytes(&self, key: String) -> Result<Option<Vec<u8>>, NBError>;
    fn remove(&self, key: String) -> Result<(), NBError>;
    fn clear(&self) -> Result<(), NBError>;
    fn has(&self, key: String) -> Result<bool, NBError>;
    fn dispose(&self) -> Result<(), NBError>;
}

pub(crate) fn read_or_cleanup<T: DeserializeOwned>(
    cache: &dyn CacheProvider,
    key: String,
) -> NBResult<Option<T>> {
    let bytes = cache.get_bytes(key.clone())?;
    if bytes.is_none() {
        return Ok(None);
    }

    let decoded = json::from_bytes(&bytes.unwrap());
    if let Ok(value) = decoded {
        return Ok(Some(value));
    }

    let _ = cache.remove(key);
    return Ok(None);
}
