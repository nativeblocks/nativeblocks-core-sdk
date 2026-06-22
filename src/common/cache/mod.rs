use crate::common::result::NbError;

#[uniffi::export(with_foreign)]
pub trait CacheProvider: Send + Sync {
    fn save_string(
        &self,
        key: String,
        value: String,
        ttl_millis: Option<i64>,
    ) -> Result<(), NbError>;
    fn get_string(&self, key: String, default: String) -> Result<String, NbError>;
    fn remove(&self, key: String) -> Result<(), NbError>;
    fn clear(&self) -> Result<(), NbError>;
    fn has(&self, key: String) -> Result<bool, NbError>;
}
