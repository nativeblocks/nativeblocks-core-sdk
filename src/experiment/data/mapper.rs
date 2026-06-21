use super::dto::NativeExperimentDataDto;
use crate::experiment::model::NativeExperimentModel;

impl NativeExperimentDataDto {
    pub(crate) fn to_model(&self) -> NativeExperimentModel {
        let (value, variable_type) = self
            .experiment_value
            .as_ref()
            .map(|e| (e.value.clone(), e.variable_type.clone()))
            .unwrap_or_default();
        NativeExperimentModel {
            value,
            variable_type,
        }
    }
}
