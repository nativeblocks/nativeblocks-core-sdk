use crate::common::result::NBResult;
use crate::experiment::data::repository::ExperimentRepository;
use crate::experiment::domain::model::{ExperimentRequest, NativeExperimentModel};

pub(crate) async fn get_experiment_use_case(
    repository: &ExperimentRepository,
    request: &ExperimentRequest,
) -> NBResult<NativeExperimentModel> {
    return repository.get_experiment(request).await;
}
