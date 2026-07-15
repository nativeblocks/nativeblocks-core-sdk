use crate::common::cache::{self, CacheProvider};
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
    return match cache::read_or_cleanup(cache, cache_key)? {
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
    let cache_key = if production { key::prod_key(route) } else { key::dev_key(route) };
    let bytes = json::to_bytes(frame)?;
    cache.save_bytes(cache_key, bytes, None)?;
    return Ok(());
}

pub(super) fn cached_checksum(cache: &dyn CacheProvider, route: &str) -> NBResult<Option<String>> {
    let frame: Option<NativeFrameModel> = cache::read_or_cleanup(cache, key::prod_key(route))?;
    return Ok(frame.and_then(|frame| frame.checksum));
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
