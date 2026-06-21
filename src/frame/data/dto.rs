use serde::Deserialize;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct NativeFrameDataDto {
    #[serde(default)]
    pub(crate) frame_production: Option<NativeFrameDto>,
    #[serde(default)]
    pub(crate) frame: Option<NativeFrameDto>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub(crate) struct NativeFrameDto {
    #[serde(default)]
    pub(crate) checksum: Option<String>,
    #[serde(default)]
    pub(crate) variables: Option<Vec<Option<NativeVariableDto>>>,
    #[serde(default)]
    pub(crate) blocks: Option<Vec<Option<NativeBlockDto>>>,
    #[serde(default)]
    pub(crate) actions: Option<Vec<Option<NativeActionDto>>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub(crate) struct NativeVariableDto {
    pub(crate) key: Option<String>,
    pub(crate) value: Option<String>,
    #[serde(rename = "type")]
    pub(crate) value_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct NativeBlockDto {
    pub(crate) id: Option<String>,
    pub(crate) parent_id: Option<String>,
    pub(crate) integration_version: Option<i32>,
    pub(crate) slot: Option<String>,
    pub(crate) key_type: Option<String>,
    pub(crate) key: Option<String>,
    pub(crate) visibility_key: Option<String>,
    pub(crate) position: Option<i32>,
    pub(crate) data: Option<Vec<Option<NativeBlockDataDto>>>,
    pub(crate) properties: Option<Vec<Option<NativeBlockPropertyDto>>>,
    pub(crate) slots: Option<Vec<Option<NativeBlockSlotDto>>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct NativeBlockPropertyDto {
    pub(crate) key: Option<String>,
    pub(crate) value_mobile: Option<String>,
    pub(crate) value_tablet: Option<String>,
    pub(crate) value_desktop: Option<String>,
    #[serde(rename = "type")]
    pub(crate) value_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub(crate) struct NativeBlockDataDto {
    pub(crate) key: Option<String>,
    pub(crate) value: Option<String>,
    #[serde(rename = "type")]
    pub(crate) value_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub(crate) struct NativeBlockSlotDto {
    pub(crate) slot: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub(crate) struct NativeActionDto {
    pub(crate) id: Option<String>,
    pub(crate) key: Option<String>,
    pub(crate) event: Option<String>,
    pub(crate) triggers: Option<Vec<Option<NativeActionTriggerDto>>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct NativeActionTriggerDto {
    pub(crate) id: Option<String>,
    pub(crate) parent_id: Option<String>,
    pub(crate) integration_version: Option<i32>,
    pub(crate) name: Option<String>,
    pub(crate) key_type: Option<String>,
    pub(crate) then: Option<String>,
    pub(crate) properties: Option<Vec<Option<NativeActionTriggerPropertyDto>>>,
    pub(crate) data: Option<Vec<Option<NativeActionTriggerDataDto>>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub(crate) struct NativeActionTriggerPropertyDto {
    pub(crate) key: Option<String>,
    pub(crate) value: Option<String>,
    #[serde(rename = "type")]
    pub(crate) value_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub(crate) struct NativeActionTriggerDataDto {
    pub(crate) key: Option<String>,
    pub(crate) value: Option<String>,
    #[serde(rename = "type")]
    pub(crate) value_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct NativeFrameProductionChecksumDataDto {
    #[serde(default)]
    pub(crate) frame_production_checksum: Option<NativeFrameProductionChecksumDto>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub(crate) struct NativeFrameProductionChecksumDto {
    pub(crate) checksum: Option<String>,
}
