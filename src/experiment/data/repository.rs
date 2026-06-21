use std::sync::Arc;
use std::time::Duration;

use crate::common::cache::CacheProvider;
use crate::common::result::{ErrorModel, NBResult};
use crate::experiment::data::source::ExperimentRemoteSource;
use crate::experiment::domain::key::{CACHE_PREFIX, CACHE_SEPARATOR, message};
use crate::experiment::domain::model::{ExperimentRequest, NativeExperimentModel};

pub(crate) struct ExperimentRepository {
    source: ExperimentRemoteSource,
    cache: Arc<dyn CacheProvider>,
    is_community: bool,
}

impl ExperimentRepository {
    pub(crate) fn new(
        source: ExperimentRemoteSource,
        cache: Arc<dyn CacheProvider>,
        is_community: bool,
    ) -> Self {
        return Self {
            source,
            cache,
            is_community,
        };
    }

    pub(crate) async fn get_experiment(
        &self,
        request: &ExperimentRequest,
    ) -> NBResult<NativeExperimentModel> {
        if self.is_community {
            return Err(ErrorModel::support(message::CLOUD_REQUIRED));
        }

        let cache_key = format!("{CACHE_PREFIX}{}", request.key);
        if self.cache.has(&cache_key).unwrap_or(false) {
            let cached = self.cache.get_string(&cache_key, "").unwrap_or_default();
            if !cached.is_empty() {
                return Ok(decode_cache_value(&cached));
            }
        }

        let dto = self.source.fetch(request).await?;
        let model = dto.to_model();
        let ttl = (request.cache_ttl_millis > 0)
            .then(|| Duration::from_millis(request.cache_ttl_millis as u64));
        let _ = self
            .cache
            .save_string(&cache_key, &encode_cache_value(&model), ttl);
        return Ok(model);
    }
}

fn encode_cache_value(model: &NativeExperimentModel) -> String {
    return format!("{}{CACHE_SEPARATOR}{}", model.value, model.variable_type);
}

fn decode_cache_value(cached: &str) -> NativeExperimentModel {
    match cached.split_once(CACHE_SEPARATOR) {
        Some((value, variable_type)) => {
            return NativeExperimentModel {
                value: value.to_string(),
                variable_type: variable_type.to_string(),
            };
        }
        None => {
            return NativeExperimentModel {
                value: String::new(),
                variable_type: String::new(),
            };
        }
    }
}
