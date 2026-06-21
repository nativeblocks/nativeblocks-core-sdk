use serde::Deserialize;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct NativeLocalizationDataDto {
    #[serde(default)]
    pub(crate) localizations_production: Option<NativeLocalizationsDto>,
    #[serde(default)]
    pub(crate) localizations: Option<NativeLocalizationsDto>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub(crate) struct NativeLocalizationsDto {
    #[serde(default)]
    pub(crate) checksum: Option<String>,
    #[serde(default)]
    pub(crate) localizations: Option<Vec<Option<NativeLocalizationDto>>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub(crate) struct NativeLocalizationDto {
    pub(crate) key: Option<String>,
    pub(crate) value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct NativeLocalizationProductionChecksumDataDto {
    #[serde(default)]
    pub(crate) localization_production_checksum: Option<NativeLocalizationProductionChecksumDto>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct NativeLocalizationProductionChecksumDto {
    pub(crate) checksum: Option<String>,
}
