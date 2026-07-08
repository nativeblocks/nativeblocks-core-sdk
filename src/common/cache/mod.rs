use crate::common::result::NBError;

#[uniffi::export(with_foreign)]
pub trait CacheProvider: Send + Sync {
    fn save_bytes(&self, key: String, value: Vec<u8>, ttl_millis: Option<i64>) -> Result<(), NBError>;
    fn get_bytes(&self, key: String) -> Result<Option<Vec<u8>>, NBError>;
    fn remove(&self, key: String) -> Result<(), NBError>;
    fn clear(&self) -> Result<(), NBError>;
    fn has(&self, key: String) -> Result<bool, NBError>;
    fn dispose(&self) -> Result<(), NBError>;
}
