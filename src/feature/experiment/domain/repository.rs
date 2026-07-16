use std::collections::HashMap;

use crate::feature::experiment::domain::model::NativeExperimentModel;
use crate::library::result::NBResult;

#[async_trait::async_trait]
pub(crate) trait ExperimentRepository: Send + Sync {
    async fn fetch(
        &self,
        key: &str,
        cache_ttl: Option<i64>,
        globals: &HashMap<String, String>,
    ) -> NBResult<NativeExperimentModel>;
}
