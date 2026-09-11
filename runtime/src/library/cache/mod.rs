use serde::de::DeserializeOwned;
use std::sync::Arc;

use crate::library::result::NBResult;

mod file_provider;
mod provider;
pub mod util;

use file_provider::FileCacheProvider;
pub(crate) use provider::CacheProvider;

pub(crate) struct Caches {
    pub(crate) config: Arc<dyn CacheProvider>,
    pub(crate) frames: Arc<dyn CacheProvider>,
    pub(crate) localizations: Arc<dyn CacheProvider>,
    pub(crate) experiments: Arc<dyn CacheProvider>,
    pub(crate) scaffold: Arc<dyn CacheProvider>,
}

pub(crate) fn build_caches(cache_dir: &str, instance_name: &str) -> NBResult<Caches> {
    let provider = |namespace: &str| -> NBResult<Arc<dyn CacheProvider>> {
        return Ok(Arc::new(FileCacheProvider::new(cache_dir, instance_name, namespace)?));
    };
    return Ok(Caches {
        config: provider("config")?,
        frames: provider("frames")?,
        localizations: provider("localizations")?,
        experiments: provider("experiments")?,
        scaffold: provider("scaffold")?,
    });
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
