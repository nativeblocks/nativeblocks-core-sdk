use crate::common::cache::CacheProvider;
use crate::common::json;
use crate::common::result::{ErrorModel, NBResult};
use crate::frame::data::key::{self, error_code};
use crate::frame::domain::model::NativeFrameModel;

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
    return match cache.get_bytes(cache_key)? {
        Some(bytes) => decode_frame(&bytes),
        None => Err(ErrorModel::cache(key::message::FRAME_NOT_CACHED).with_code(error_code::FRAME_NOT_CACHED)),
    };
}

pub(super) fn save_frame(
    cache: &dyn CacheProvider,
    route: &str,
    frame: &NativeFrameModel,
    production: bool,
) -> NBResult<()> {
    let cache_key = if production { key::prod_key(route) } else { key::dev_key(route) };
    let bytes = json::to_bytes(frame)?;
    cache.save_bytes(cache_key, bytes, None)?;
    return Ok(());
}

pub(super) fn cached_checksum(cache: &dyn CacheProvider, route: &str) -> NBResult<Option<String>> {
    let bytes = cache.get_bytes(key::prod_key(route))?;
    if bytes.is_none() {
        return Ok(None);
    }
    // An unreadable cached frame (schema drift from an older SDK build)
    // counts as not cached: reporting None makes the caller do a full
    // fetch, which overwrites the stale bytes with the current shape.
    return match decode_frame(&bytes.unwrap()) {
        Ok(frame) => Ok(frame.checksum),
        Err(_) => Ok(None),
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

fn decode_frame(bytes: &[u8]) -> NBResult<NativeFrameModel> {
    return json::from_bytes(bytes).map_err(|error| error.with_code(error_code::FRAME_NOT_CACHED));
}
