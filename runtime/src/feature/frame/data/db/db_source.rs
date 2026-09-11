use rkyv::rancor::Error as RkyvError;

use crate::feature::frame::data::key::{self, error_code};
use crate::feature::frame::domain::model::NativeFrameModel;
use crate::library::cache::CacheProvider;
use crate::library::result::{ErrorModel, NBResult};

pub(in crate::feature::frame::data) async fn get_frame(
    cache: &dyn CacheProvider,
    route: &str,
    development_mode: bool,
) -> NBResult<NativeFrameModel> {
    let cache_key = if development_mode {
        key::dev_key(route)
    } else {
        key::prod_key(route)
    };
    return match read_frame(cache, cache_key).await? {
        Some(frame) => Ok(frame),
        None => Err(not_cached()),
    };
}

pub(in crate::feature::frame::data) fn not_cached() -> ErrorModel {
    return ErrorModel::cache(key::message::FRAME_NOT_CACHED)
        .with_code(error_code::FRAME_NOT_CACHED);
}

pub(in crate::feature::frame::data) async fn save_frame(
    cache: &dyn CacheProvider,
    route: &str,
    frame: &NativeFrameModel,
    production: bool,
) -> NBResult<()> {
    let cache_key = if production {
        key::prod_key(route)
    } else {
        key::dev_key(route)
    };
    let bytes = rkyv::to_bytes::<RkyvError>(frame)
        .map_err(|error| ErrorModel::cache(error.to_string()))?
        .to_vec();
    cache.save(cache_key, bytes, None).await?;
    if production {
        let checksum = frame.checksum.clone().unwrap_or_default();
        cache
            .save(key::prod_checksum_key(route), checksum.into_bytes(), None)
            .await?;
    }
    return Ok(());
}

pub(in crate::feature::frame::data) async fn cached_checksum(
    cache: &dyn CacheProvider,
    route: &str,
) -> NBResult<Option<String>> {
    let Some(bytes) = cache.get(key::prod_checksum_key(route)).await? else {
        return Ok(None);
    };
    return Ok(String::from_utf8(bytes)
        .ok()
        .filter(|checksum| !checksum.is_empty()));
}

async fn read_frame(cache: &dyn CacheProvider, key: String) -> NBResult<Option<NativeFrameModel>> {
    let Some(bytes) = cache.get(key.clone()).await? else {
        return Ok(None);
    };
    let mut aligned = rkyv::util::AlignedVec::<16>::new();
    aligned.extend_from_slice(&bytes);

    return match rkyv::from_bytes::<NativeFrameModel, RkyvError>(aligned.as_slice()) {
        Ok(frame) => Ok(Some(frame)),
        Err(_) => {
            let _ = cache.remove(key).await;
            Ok(None)
        }
    };
}

pub(in crate::feature::frame::data) async fn clear(
    cache: &dyn CacheProvider,
    route: &str,
) -> NBResult<()> {
    cache.remove(key::dev_key(route)).await?;
    cache.remove(key::prod_key(route)).await?;
    cache.remove(key::prod_checksum_key(route)).await?;
    return Ok(());
}

pub(in crate::feature::frame::data) async fn clear_all(cache: &dyn CacheProvider) -> NBResult<()> {
    cache.clear().await?;
    return Ok(());
}
