use crate::feature::embedded::SeedError;
use crate::feature::localization::data::db_source;
use crate::feature::localization::data::dto::NativeLocalizationsDto;
use crate::feature::localization::data::key;
use crate::feature::localization::data::mapper;
use crate::library::cache::CacheProvider;
use crate::library::net::decode_binary;

pub(crate) async fn seed(
    cache: &dyn CacheProvider,
    language_code: &str,
    file: &[u8],
    checksum: &str,
) -> Result<bool, SeedError> {
    let cached = cache
        .has(key::prod_key(language_code))
        .await
        .map_err(|error| SeedError::Cache(error.into()))?;
    if cached {
        return Ok(false);
    }
    let payload =
        decode_binary(&key::BINARY_FORMAT, file, language_code).map_err(SeedError::Invalid)?;
    let dto: NativeLocalizationsDto = serde_json::from_slice(&payload)
        .map_err(|error| SeedError::Invalid(format!("invalid localization payload: {error}")))?;
    let localization = mapper::to_model(Some(&dto));
    if localization.checksum.as_deref() != Some(checksum) {
        return Err(SeedError::Invalid(
            "localization checksum does not match the manifest".to_string(),
        ));
    }
    db_source::save_localization(cache, language_code, &localization, true)
        .await
        .map_err(SeedError::Cache)?;
    return Ok(true);
}
