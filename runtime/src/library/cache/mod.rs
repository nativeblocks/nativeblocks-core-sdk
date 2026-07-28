use serde::de::DeserializeOwned;
use std::sync::Arc;

use crate::library::result::NBResult;

mod file_provider;
mod provider;
pub mod util;

use file_provider::FileCacheProvider;
pub(crate) use provider::CacheProvider;

pub(crate) fn build_provider(
    cache_dir: &str,
    instance_name: &str,
) -> NBResult<Arc<dyn CacheProvider>> {
    return Ok(Arc::new(FileCacheProvider::new(cache_dir, instance_name)?));
}

pub(crate) async fn read_or_cleanup<T: DeserializeOwned>(
    cache: &dyn CacheProvider,
    key: String,
) -> NBResult<Option<T>> {
    let bytes = cache.get(key.clone()).await?;
    if bytes.is_none() {
        return Ok(None);
    }

    let decoded = util::from_bytes(&bytes.unwrap());
    if let Ok(value) = decoded {
        return Ok(Some(value));
    }

    let _ = cache.remove(key).await;
    return Ok(None);
}
