use crate::library::result::NBError;

/// The cache contract used across the runtime. Internal to Rust: the host has no
/// say in where or how records are stored.
///
/// Implementations are expected to keep filesystem work off the async worker
/// threads, so every method is awaited by its callers.
#[async_trait::async_trait]
pub(crate) trait CacheProvider: Send + Sync {
    /// Stores `value` under `key`, replacing whatever was there. The replacement
    /// becomes visible only once the new content is completely written, so a
    /// reader sees either the previous record or the new one, never a partial.
    async fn save(
        &self,
        key: String,
        value: Vec<u8>,
        ttl_millis: Option<i64>,
    ) -> Result<(), NBError>;

    /// Returns the stored value, or `None` when the record is absent, expired,
    /// or unreadable. Expired and unreadable records are dropped on the way out.
    async fn get(&self, key: String) -> Result<Option<Vec<u8>>, NBError>;

    /// Deletes the record for `key`. Succeeds whether or not it existed.
    async fn remove(&self, key: String) -> Result<(), NBError>;

    /// Reports whether a live record exists, without reading its value. Expired
    /// records are dropped and reported as absent.
    async fn has(&self, key: String) -> Result<bool, NBError>;
}
