use crate::feature::localization::data::key::{self, error_code};
use crate::feature::localization::domain::model::NativeLocalizationModel;
use crate::library::cache::util;
use crate::library::cache::{self, CacheProvider};
use crate::library::result::{ErrorModel, NBResult};

pub(super) async fn get_localization(
    cache: &dyn CacheProvider,
    language_code: &str,
    development_mode: bool,
) -> NBResult<NativeLocalizationModel> {
    let cache_key = if development_mode {
        key::dev_key(language_code)
    } else {
        key::prod_key(language_code)
    };
    return match cache::read_or_cleanup(cache, cache_key).await? {
        Some(localization) => Ok(localization),
        None => Err(ErrorModel::cache(key::message::LOCALIZATION_NOT_CACHED)
            .with_code(error_code::LOCALIZATION_NOT_CACHED)),
    };
}

pub(super) async fn save_localization(
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
    let bytes = util::to_bytes(localization)?;
    cache.save(cache_key, bytes, None).await?;
    return Ok(());
}

pub(super) async fn cached_checksum(
    cache: &dyn CacheProvider,
    language_code: &str,
) -> NBResult<Option<String>> {
    let localization: Option<NativeLocalizationModel> =
        cache::read_or_cleanup(cache, key::prod_key(language_code)).await?;
    return Ok(localization.and_then(|localization| localization.checksum));
}
