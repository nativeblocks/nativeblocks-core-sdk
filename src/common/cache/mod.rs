#[cfg(feature = "cache-sqlite")]
pub mod sqlite;

use std::sync::Arc;
use std::time::Duration;
use crate::common::result::NBResult;

#[cfg(feature = "cache-sqlite")]
pub fn new_cache_provider(path: &str) -> NBResult<Arc<dyn CacheProvider>> {
    return Ok(Arc::new(sqlite::SqliteCacheProvider::open(path)?));
}

pub trait CacheProvider: Send + Sync {
    fn save_string(&self, key: &str, value: &str, ttl: Option<Duration>) -> NBResult<()>;

    fn get_string(&self, key: &str, default: &str) -> NBResult<String>;

    fn remove(&self, key: &str) -> NBResult<()>;

    fn clear(&self) -> NBResult<()>;

    fn has(&self, key: &str) -> NBResult<bool>;

    fn save_bool(&self, key: &str, value: bool, ttl: Option<Duration>) -> NBResult<()> {
        self.save_string(key, &value.to_string(), ttl)
    }

    fn save_i64(&self, key: &str, value: i64, ttl: Option<Duration>) -> NBResult<()> {
        self.save_string(key, &value.to_string(), ttl)
    }

    fn save_f64(&self, key: &str, value: f64, ttl: Option<Duration>) -> NBResult<()> {
        self.save_string(key, &value.to_string(), ttl)
    }

    fn get_bool(&self, key: &str, default: bool) -> NBResult<bool> {
        let raw = self.get_string(key, &default.to_string())?;
        Ok(raw.parse().unwrap_or(default))
    }

    fn get_i64(&self, key: &str, default: i64) -> NBResult<i64> {
        let raw = self.get_string(key, &default.to_string())?;
        Ok(raw.parse().unwrap_or(default))
    }

    fn get_f64(&self, key: &str, default: f64) -> NBResult<f64> {
        let raw = self.get_string(key, &default.to_string())?;
        Ok(raw.parse().unwrap_or(default))
    }
}
