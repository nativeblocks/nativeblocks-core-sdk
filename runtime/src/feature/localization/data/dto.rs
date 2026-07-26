use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct NativeLocalizationDataDto {
    pub(super) localizations: Option<NativeLocalizationsDto>,
    pub(super) localizations_production: Option<NativeLocalizationsDto>,
}

#[derive(Deserialize)]
pub(super) struct NativeLocalizationsDto {
    pub(super) checksum: Option<String>,
    pub(super) localizations: Option<Vec<NativeLocalizationItemDto>>,
}

#[derive(Deserialize)]
pub(super) struct NativeLocalizationItemDto {
    pub(super) key: Option<String>,
    pub(super) value: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct NativeLocalizationProductionChecksumDataDto {
    pub(super) localization_production_checksum: Option<NativeLocalizationProductionChecksumDto>,
}

#[derive(Deserialize)]
pub(super) struct NativeLocalizationProductionChecksumDto {
    pub(super) checksum: Option<String>,
}
