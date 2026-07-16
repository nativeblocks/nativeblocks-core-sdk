pub(super) const GATEWAY_LOCALIZATION: &str = "localizations";
pub(super) const GATEWAY_LOCALIZATION_PRODUCTION: &str = "localizationsProduction";
pub(super) const GATEWAY_LOCALIZATION_PRODUCTION_CHECKSUM: &str = "localizationProductionChecksum";

pub(super) const PARAM_LANGUAGE_CODE: &str = "languageCode";

pub(super) fn dev_key(language_code: &str) -> String {
    return format!("LOCALIZATION_DEV::{language_code}");
}

pub(super) fn prod_key(language_code: &str) -> String {
    return format!("LOCALIZATION_PROD::{language_code}");
}

pub(in crate::feature::localization) mod error_code {
    pub(in crate::feature::localization) const LOCALIZATION_NOT_CACHED: &str = "NB0201";
    pub(in crate::feature::localization) const LOCALIZATION_PRODUCTION_SYNC: &str = "NB0202";
    pub(in crate::feature::localization) const LOCALIZATION_DEV_SYNC: &str = "NB0203";
    pub(in crate::feature::localization) const LOCALIZATION_CHECKSUM: &str = "NB0204";
}

pub(in crate::feature::localization) mod message {
    pub(in crate::feature::localization) const LOCALIZATION_NOT_CACHED: &str =
        "Please make sure internet connection is available";
}
