use crate::common::result::NbError;

mod migration;

pub(crate) use migration::run_migrations;

#[uniffi::export(with_foreign)]
pub trait CacheProvider: Send + Sync {
    fn save_bytes(&self, key: String, value: Vec<u8>, ttl_millis: Option<i64>) -> Result<(), NbError>;
    fn get_bytes(&self, key: String) -> Result<Option<Vec<u8>>, NbError>;
    fn remove(&self, key: String) -> Result<(), NbError>;
    fn clear(&self) -> Result<(), NbError>;
    fn has(&self, key: String) -> Result<bool, NbError>;
}
