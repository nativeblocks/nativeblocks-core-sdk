use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct NativeExperimentDataDto {
    pub(crate) experiment_value: Option<NativeExperimentDto>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct NativeExperimentDto {
    #[serde(default)]
    pub(crate) value: String,
    #[serde(default)]
    pub(crate) variable_type: String,
}
