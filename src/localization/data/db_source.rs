use crate::common::cache::CacheProvider;
use crate::common::json;
use crate::common::result::{ErrorModel, NBResult};
use crate::localization::data::key::{self, error_code};
use crate::localization::domain::model::NativeLocalizationModel;

pub(super) fn get_localization(
    cache: &dyn CacheProvider,
    language_code: &str,
    development_mode: bool,
) -> NBResult<NativeLocalizationModel> {
    let cache_key = if development_mode {
        key::dev_key(language_code)
    } else {
        key::prod_key(language_code)
    };
    return match cache.get_bytes(cache_key)? {
        Some(bytes) => decode_localization(&bytes),
        None => Err(ErrorModel::cache(key::message::LOCALIZATION_NOT_CACHED)
            .with_code(error_code::LOCALIZATION_NOT_CACHED)),
    };
}

pub(super) fn save_localization(
    cache: &dyn CacheProvider,
    language_code: &str,
    localization: &NativeLocalizationModel,
    production: bool,
) -> NBResult<()> {
    let cache_key = if production {
        key::prod_key(language_code)
    } else {
        key::dev_key(language_code)
    };
    let bytes = json::to_bytes(localization)?;
    cache.save_bytes(cache_key, bytes, None)?;
    return Ok(());
}

pub(super) fn cached_checksum(cache: &dyn CacheProvider, language_code: &str) -> NBResult<Option<String>> {
    let bytes = cache.get_bytes(key::prod_key(language_code))?;
    if bytes.is_none() {
        return Ok(None);
    }
    // An unreadable cached localization (schema drift from an older SDK
    // build) counts as not cached: reporting None makes the caller do a
    // full fetch, which overwrites the stale bytes with the current shape.
    return match decode_localization(&bytes.unwrap()) {
        Ok(localization) => Ok(localization.checksum),
        Err(_) => Ok(None),
    };
}

fn decode_localization(bytes: &[u8]) -> NBResult<NativeLocalizationModel> {
    return json::from_bytes(bytes)
        .map_err(|error| error.with_code(error_code::LOCALIZATION_NOT_CACHED));
}
