use rkyv::rancor::Error as RkyvError;

use crate::feature::frame::data::key::{self, error_code};
use crate::feature::frame::domain::model::NativeFrameModel;
use crate::library::cache::CacheProvider;
use crate::library::result::{ErrorModel, NBResult};

pub(super) fn get_frame(
    cache: &dyn CacheProvider,
    route: &str,
    development_mode: bool,
) -> NBResult<NativeFrameModel> {
    let cache_key = if development_mode {
        key::dev_key(route)
    } else {
        key::prod_key(route)
    };
    return match read_frame(cache, cache_key)? {
        Some(frame) => Ok(frame),
        None => Err(not_cached()),
    };
}

pub(super) fn not_cached() -> ErrorModel {
    return ErrorModel::cache(key::message::FRAME_NOT_CACHED)
        .with_code(error_code::FRAME_NOT_CACHED);
}

pub(super) fn save_frame(
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
    cache.save_bytes(cache_key, bytes, None)?;
    return Ok(());
}

pub(super) fn cached_checksum(cache: &dyn CacheProvider, route: &str) -> NBResult<Option<String>> {
    let frame = read_frame(cache, key::prod_key(route))?;
    return Ok(frame.and_then(|frame| frame.checksum));
}

fn read_frame(cache: &dyn CacheProvider, key: String) -> NBResult<Option<NativeFrameModel>> {
    let Some(bytes) = cache.get_bytes(key.clone())? else {
        return Ok(None);
    };
    let mut aligned = rkyv::util::AlignedVec::<16>::new();
    aligned.extend_from_slice(&bytes);

    return match rkyv::from_bytes::<NativeFrameModel, RkyvError>(aligned.as_slice()) {
        Ok(frame) => Ok(Some(frame)),
        Err(_) => {
            let _ = cache.remove(key);
            Ok(None)
        }
    };
}

pub(super) async fn clear(cache: &dyn CacheProvider, route: &str) -> NBResult<()> {
    cache.remove(key::dev_key(route))?;
    cache.remove(key::prod_key(route))?;
    return Ok(());
}

pub(super) async fn clear_all(cache: &dyn CacheProvider, routes: &[String]) -> NBResult<()> {
    for route in routes {
        clear(cache, route).await?;
    }
    return Ok(());
}
