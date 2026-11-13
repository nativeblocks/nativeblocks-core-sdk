use crate::feature::embedded::SeedError;
use crate::feature::scaffold::data::key;
use crate::feature::scaffold::domain::model::ScaffoldModel;
use crate::library::cache::CacheProvider;
use crate::library::cache::util;

pub(crate) async fn seed(
    cache: &dyn CacheProvider,
    scaffold: &ScaffoldModel,
) -> Result<bool, SeedError> {
    let cache_key = key::cache_key(false).to_string();
    let cached = cache
        .has(cache_key.clone())
        .await
        .map_err(|error| SeedError::Cache(error.into()))?;
    if cached {
        return Ok(false);
    }
    let bytes = util::to_bytes(scaffold).map_err(SeedError::Cache)?;
    cache
        .save(cache_key, bytes, None)
        .await
        .map_err(|error| SeedError::Cache(error.into()))?;
    return Ok(true);
}
