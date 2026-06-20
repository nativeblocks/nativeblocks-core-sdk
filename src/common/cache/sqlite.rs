use std::sync::Mutex;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use rusqlite::{Connection, OptionalExtension, params};

use super::CacheProvider;
use crate::common::result::{ErrorModel, NbResult};

pub struct SqliteCacheProvider {
    conn: Mutex<Connection>,
}

impl SqliteCacheProvider {
    pub fn open(path: &str) -> NbResult<Self> {
        let conn = Connection::open(path).map_err(map_sqlite_error)?;
        Self::init(conn)
    }

    pub fn in_memory() -> NbResult<Self> {
        let conn = Connection::open_in_memory().map_err(map_sqlite_error)?;
        Self::init(conn)
    }

    fn init(conn: Connection) -> NbResult<Self> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS cache (
                key    TEXT PRIMARY KEY,
                value  TEXT NOT NULL,
                expiry INTEGER
            )",
            [],
        )
        .map_err(map_sqlite_error)?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    fn lock(&self) -> NbResult<std::sync::MutexGuard<'_, Connection>> {
        self.conn
            .lock()
            .map_err(|_| ErrorModel::cache("Cache connection poisoned"))
    }
}

impl CacheProvider for SqliteCacheProvider {
    fn save_string(&self, key: &str, value: &str, ttl: Option<Duration>) -> NbResult<()> {
        let expiry = ttl.map(|d| now_millis().saturating_add(d.as_millis() as i64));
        let conn = self.lock()?;
        conn.execute(
            "INSERT INTO cache (key, value, expiry) VALUES (?1, ?2, ?3)
             ON CONFLICT(key) DO UPDATE SET value = ?2, expiry = ?3",
            params![key, value, expiry],
        )
        .map_err(map_sqlite_error)?;
        Ok(())
    }

    fn get_string(&self, key: &str, default: &str) -> NbResult<String> {
        let conn = self.lock()?;
        let row: Option<(String, Option<i64>)> = conn
            .query_row(
                "SELECT value, expiry FROM cache WHERE key = ?1",
                params![key],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .optional()
            .map_err(map_sqlite_error)?;

        match row {
            Some((value, expiry)) => {
                if expiry.is_some_and(|e| now_millis() > e) {
                    // Expired: drop the row and fall back (mirrors `isExpired`).
                    conn.execute("DELETE FROM cache WHERE key = ?1", params![key])
                        .map_err(map_sqlite_error)?;
                    Ok(default.to_string())
                } else {
                    Ok(value)
                }
            }
            None => Ok(default.to_string()),
        }
    }

    fn remove(&self, key: &str) -> NbResult<()> {
        self.lock()?
            .execute("DELETE FROM cache WHERE key = ?1", params![key])
            .map_err(map_sqlite_error)?;
        Ok(())
    }

    fn clear(&self) -> NbResult<()> {
        self.lock()?
            .execute("DELETE FROM cache", [])
            .map_err(map_sqlite_error)?;
        Ok(())
    }

    fn has(&self, key: &str) -> NbResult<bool> {
        let conn = self.lock()?;
        conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM cache WHERE key = ?1 AND (expiry IS NULL OR expiry > ?2))",
            params![key, now_millis()],
            |r| r.get(0),
        )
        .map_err(map_sqlite_error)
    }
}

fn now_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

fn map_sqlite_error(error: rusqlite::Error) -> ErrorModel {
    ErrorModel::cache(error.to_string())
}

#[cfg(test)]
#[path = "sqlite.test.rs"]
mod tests;
