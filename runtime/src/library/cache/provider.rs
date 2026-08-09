use crate::library::result::NBError;

#[async_trait::async_trait]
pub(crate) trait CacheProvider: Send + Sync {
    async fn save(
        &self,
        key: String,
        value: Vec<u8>,
        ttl_millis: Option<i64>,
    ) -> Result<(), NBError>;

    async fn get(&self, key: String) -> Result<Option<Vec<u8>>, NBError>;

    async fn remove(&self, key: String) -> Result<(), NBError>;

    async fn has(&self, key: String) -> Result<bool, NBError>;
}
