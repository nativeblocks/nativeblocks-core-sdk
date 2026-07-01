use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct NativeExperimentDataDto {
    pub(super) experiment_value: Option<NativeExperimentDto>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct NativeExperimentDto {
    pub(super) value: Option<String>,
    pub(super) variable_type: Option<String>,
}
