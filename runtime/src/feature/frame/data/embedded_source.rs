use crate::feature::embedded::SeedError;
use crate::feature::frame::data::db::db_source;
use crate::feature::frame::data::key;
use crate::feature::frame::data::mapper;
use crate::feature::frame::data::network::dto::NativeFrameDto;
use crate::library::cache::CacheProvider;
use crate::library::net::decode_binary;

pub(crate) async fn seed(
    cache: &dyn CacheProvider,
    route: &str,
    file: &[u8],
    checksum: &str,
) -> Result<bool, SeedError> {
    let cached = cache
        .has(key::prod_key(route))
        .await
        .map_err(|error| SeedError::Cache(error.into()))?;
    if cached {
        return Ok(false);
    }
    let payload = decode_binary(&key::BINARY_FORMAT, file, route).map_err(SeedError::Invalid)?;
    let dto: NativeFrameDto = serde_json::from_slice(&payload)
        .map_err(|error| SeedError::Invalid(format!("invalid frame payload: {error}")))?;
    let frame = mapper::to_model(Some(dto));
    if frame.checksum.as_deref() != Some(checksum) {
        return Err(SeedError::Invalid(
            "frame checksum does not match the manifest".to_string(),
        ));
    }
    db_source::save_frame(cache, route, &frame, true)
        .await
        .map_err(SeedError::Cache)?;
    return Ok(true);
}
