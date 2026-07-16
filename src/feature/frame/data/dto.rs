use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct NativeFrameDataDto {
    pub(super) frame: Option<NativeFrameDto>,
    pub(super) frame_production: Option<NativeFrameDto>,
}

#[derive(Deserialize)]
pub(super) struct NativeFrameDto {
    pub(super) checksum: Option<String>,
    pub(super) variables: Option<Vec<NativeVariableDto>>,
    pub(super) blocks: Option<Vec<NativeBlockDto>>,
    pub(super) actions: Option<Vec<NativeActionDto>>,
}

#[derive(Deserialize)]
pub(super) struct NativeVariableDto {
    pub(super) key: Option<String>,
    pub(super) value: Option<String>,
    #[serde(rename = "type")]
    pub(super) variable_type: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct NativeBlockDto {
    pub(super) id: Option<String>,
    pub(super) parent_id: Option<String>,
    pub(super) integration_version: Option<i32>,
    pub(super) slot: Option<String>,
    pub(super) key_type: Option<String>,
    pub(super) key: Option<String>,
    pub(super) visibility_key: Option<String>,
    pub(super) position: Option<i32>,
    pub(super) data: Option<Vec<NativeBlockDataDto>>,
    pub(super) properties: Option<Vec<NativeBlockPropertyDto>>,
    pub(super) slots: Option<Vec<NativeBlockSlotDto>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct NativeBlockPropertyDto {
    pub(super) key: Option<String>,
    pub(super) value_mobile: Option<String>,
    pub(super) value_tablet: Option<String>,
    pub(super) value_desktop: Option<String>,
    #[serde(rename = "type")]
    pub(super) property_type: Option<String>,
}

#[derive(Deserialize)]
pub(super) struct NativeBlockDataDto {
    pub(super) key: Option<String>,
    pub(super) value: Option<String>,
    #[serde(rename = "type")]
    pub(super) data_type: Option<String>,
}

#[derive(Deserialize)]
pub(super) struct NativeBlockSlotDto {
    pub(super) slot: Option<String>,
}

#[derive(Deserialize)]
pub(super) struct NativeActionDto {
    pub(super) id: Option<String>,
    pub(super) key: Option<String>,
    pub(super) event: Option<String>,
    pub(super) triggers: Option<Vec<NativeActionTriggerDto>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct NativeActionTriggerDto {
    pub(super) id: Option<String>,
    pub(super) parent_id: Option<String>,
    pub(super) integration_version: Option<i32>,
    pub(super) name: Option<String>,
    pub(super) key_type: Option<String>,
    pub(super) then: Option<String>,
    pub(super) properties: Option<Vec<NativeActionTriggerPropertyDto>>,
    pub(super) data: Option<Vec<NativeActionTriggerDataDto>>,
}

#[derive(Deserialize)]
pub(super) struct NativeActionTriggerPropertyDto {
    pub(super) key: Option<String>,
    pub(super) value: Option<String>,
    #[serde(rename = "type")]
    pub(super) property_type: Option<String>,
}

#[derive(Deserialize)]
pub(super) struct NativeActionTriggerDataDto {
    pub(super) key: Option<String>,
    pub(super) value: Option<String>,
    #[serde(rename = "type")]
    pub(super) data_type: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct NativeFrameProductionChecksumDataDto {
    pub(super) frame_production_checksum: Option<NativeFrameProductionChecksumDto>,
}

#[derive(Deserialize)]
pub(super) struct NativeFrameProductionChecksumDto {
    pub(super) checksum: Option<String>,
}
