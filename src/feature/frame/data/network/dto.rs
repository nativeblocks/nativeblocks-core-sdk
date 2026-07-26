use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::feature::frame::data) struct NativeFrameDataDto {
    pub(in crate::feature::frame::data) frame: Option<NativeFrameDto>,
    pub(in crate::feature::frame::data) frame_production: Option<NativeFrameDto>,
}

#[derive(Deserialize)]
pub(in crate::feature::frame::data) struct NativeFrameDto {
    pub(in crate::feature::frame::data) checksum: Option<String>,
    pub(in crate::feature::frame::data) variables: Option<Vec<NativeVariableDto>>,
    pub(in crate::feature::frame::data) blocks: Option<Vec<NativeBlockDto>>,
    pub(in crate::feature::frame::data) actions: Option<Vec<NativeActionDto>>,
}

#[derive(Deserialize)]
pub(in crate::feature::frame::data) struct NativeVariableDto {
    pub(in crate::feature::frame::data) key: Option<String>,
    pub(in crate::feature::frame::data) value: Option<String>,
    #[serde(rename = "type")]
    pub(in crate::feature::frame::data) variable_type: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::feature::frame::data) struct NativeBlockDto {
    pub(in crate::feature::frame::data) id: Option<String>,
    pub(in crate::feature::frame::data) parent_id: Option<String>,
    pub(in crate::feature::frame::data) integration_version: Option<i32>,
    pub(in crate::feature::frame::data) slot: Option<String>,
    pub(in crate::feature::frame::data) key_type: Option<String>,
    pub(in crate::feature::frame::data) key: Option<String>,
    pub(in crate::feature::frame::data) visibility_key: Option<String>,
    pub(in crate::feature::frame::data) position: Option<i32>,
    pub(in crate::feature::frame::data) data: Option<Vec<NativeBlockDataDto>>,
    pub(in crate::feature::frame::data) properties: Option<Vec<NativeBlockPropertyDto>>,
    pub(in crate::feature::frame::data) slots: Option<Vec<NativeBlockSlotDto>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::feature::frame::data) struct NativeBlockPropertyDto {
    pub(in crate::feature::frame::data) key: Option<String>,
    pub(in crate::feature::frame::data) value_mobile: Option<String>,
    pub(in crate::feature::frame::data) value_tablet: Option<String>,
    pub(in crate::feature::frame::data) value_desktop: Option<String>,
    #[serde(rename = "type")]
    pub(in crate::feature::frame::data) property_type: Option<String>,
}

#[derive(Deserialize)]
pub(in crate::feature::frame::data) struct NativeBlockDataDto {
    pub(in crate::feature::frame::data) key: Option<String>,
    pub(in crate::feature::frame::data) value: Option<String>,
    #[serde(rename = "type")]
    pub(in crate::feature::frame::data) data_type: Option<String>,
}

#[derive(Deserialize)]
pub(in crate::feature::frame::data) struct NativeBlockSlotDto {
    pub(in crate::feature::frame::data) slot: Option<String>,
}

#[derive(Deserialize)]
pub(in crate::feature::frame::data) struct NativeActionDto {
    pub(in crate::feature::frame::data) id: Option<String>,
    pub(in crate::feature::frame::data) key: Option<String>,
    pub(in crate::feature::frame::data) event: Option<String>,
    pub(in crate::feature::frame::data) triggers: Option<Vec<NativeActionTriggerDto>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::feature::frame::data) struct NativeActionTriggerDto {
    pub(in crate::feature::frame::data) id: Option<String>,
    pub(in crate::feature::frame::data) parent_id: Option<String>,
    pub(in crate::feature::frame::data) integration_version: Option<i32>,
    pub(in crate::feature::frame::data) name: Option<String>,
    pub(in crate::feature::frame::data) key_type: Option<String>,
    pub(in crate::feature::frame::data) then: Option<String>,
    pub(in crate::feature::frame::data) properties: Option<Vec<NativeActionTriggerPropertyDto>>,
    pub(in crate::feature::frame::data) data: Option<Vec<NativeActionTriggerDataDto>>,
}

#[derive(Deserialize)]
pub(in crate::feature::frame::data) struct NativeActionTriggerPropertyDto {
    pub(in crate::feature::frame::data) key: Option<String>,
    pub(in crate::feature::frame::data) value: Option<String>,
    #[serde(rename = "type")]
    pub(in crate::feature::frame::data) property_type: Option<String>,
}

#[derive(Deserialize)]
pub(in crate::feature::frame::data) struct NativeActionTriggerDataDto {
    pub(in crate::feature::frame::data) key: Option<String>,
    pub(in crate::feature::frame::data) value: Option<String>,
    #[serde(rename = "type")]
    pub(in crate::feature::frame::data) data_type: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::feature::frame::data) struct NativeFrameProductionChecksumDataDto {
    pub(in crate::feature::frame::data) frame_production_checksum: Option<NativeFrameProductionChecksumDto>,
}

#[derive(Deserialize)]
pub(in crate::feature::frame::data) struct NativeFrameProductionChecksumDto {
    pub(in crate::feature::frame::data) checksum: Option<String>,
}
