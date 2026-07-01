use crate::experiment::data::dto::NativeExperimentDataDto;
use crate::experiment::domain::model::NativeExperimentModel;

pub(super) fn to_model(dto: &NativeExperimentDataDto) -> NativeExperimentModel {
    let experiment = dto.experiment_value.as_ref();
    return NativeExperimentModel {
        value: experiment
            .and_then(|it| it.value.clone())
            .unwrap_or_default(),
        variable_type: experiment
            .and_then(|it| it.variable_type.clone())
            .unwrap_or_default(),
    };
}
