#[cfg(feature = "cache-sqlite")]
pub mod sqlite;

use std::time::Duration;
use crate::common::result::NbResult;

pub trait CacheProvider: Send + Sync {
    fn save_string(&self, key: &str, value: &str, ttl: Option<Duration>) -> NbResult<()>;

    fn get_string(&self, key: &str, default: &str) -> NbResult<String>;

    fn remove(&self, key: &str) -> NbResult<()>;

    fn clear(&self) -> NbResult<()>;

    fn has(&self, key: &str) -> NbResult<bool>;

    fn save_bool(&self, key: &str, value: bool, ttl: Option<Duration>) -> NbResult<()> {
        self.save_string(key, &value.to_string(), ttl)
    }

    fn save_i64(&self, key: &str, value: i64, ttl: Option<Duration>) -> NbResult<()> {
        self.save_string(key, &value.to_string(), ttl)
    }

    fn save_f64(&self, key: &str, value: f64, ttl: Option<Duration>) -> NbResult<()> {
        self.save_string(key, &value.to_string(), ttl)
    }

    fn get_bool(&self, key: &str, default: bool) -> NbResult<bool> {
        let raw = self.get_string(key, &default.to_string())?;
        Ok(raw.parse().unwrap_or(default))
    }

    fn get_i64(&self, key: &str, default: i64) -> NbResult<i64> {
        let raw = self.get_string(key, &default.to_string())?;
        Ok(raw.parse().unwrap_or(default))
    }

    fn get_f64(&self, key: &str, default: f64) -> NbResult<f64> {
        let raw = self.get_string(key, &default.to_string())?;
        Ok(raw.parse().unwrap_or(default))
    }
}
