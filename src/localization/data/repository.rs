use std::sync::Arc;

use crate::common::result::{ErrorModel, NBResult};
use crate::localization::data::remote::LocalizationRemoteSource;
use crate::localization::data::source::LocalizationLocalSource;
use crate::localization::domain::key::{error_code, message};
use crate::localization::domain::model::{LocalizationSyncRequest, NativeLocalizationModel};

pub(crate) struct LocalizationRepository {
    remote: LocalizationRemoteSource,
    local: Arc<dyn LocalizationLocalSource>,
    development_mode: bool,
}

impl LocalizationRepository {
    pub(crate) fn new(
        remote: LocalizationRemoteSource,
        local: Arc<dyn LocalizationLocalSource>,
        development_mode: bool,
    ) -> Self {
        return Self {
            remote,
            local,
            development_mode,
        };
    }

    pub(crate) async fn sync(
        &self,
        request: &LocalizationSyncRequest,
    ) -> NBResult<Option<NativeLocalizationModel>> {
        let lang = &request.language_code;

        if self.development_mode {
            let model = self
                .remote
                .fetch(
                    &request.endpoint_localization,
                    &request.graphql_endpoint,
                    lang,
                    &request.install_id,
                    false,
                )
                .await
                .map_err(|e| with_code(e, error_code::SYNC))?;
            let json = serialize(&model)?;
            self.local
                .dev_upsert(lang, model.checksum.as_deref().unwrap_or(""), &json)?;
            return Ok(Some(model));
        }

        let remote_checksum = self
            .remote
            .fetch_checksum(
                &request.endpoint_localization_production_checksum,
                &request.graphql_endpoint,
                lang,
                &request.install_id,
            )
            .await
            .map_err(|e| with_code(e, error_code::CHECKSUM))?;
        let cached = self.local.prod_find_checksum(lang, &remote_checksum)?;
        if remote_checksum == cached.unwrap_or_default() {
            return Ok(None);
        }

        let model = self
            .remote
            .fetch(
                &request.endpoint_localization_production,
                &request.graphql_endpoint,
                lang,
                &request.install_id,
                true,
            )
            .await
            .map_err(|e| with_code(e, error_code::PRODUCTION_SYNC))?;
        let json = serialize(&model)?;
        self.local
            .prod_upsert(lang, model.checksum.as_deref().unwrap_or(""), &json)?;
        return Ok(Some(model));
    }

    pub(crate) async fn load(&self, language_code: &str) -> NBResult<NativeLocalizationModel> {
        let json = if self.development_mode {
            self.local.dev_find_by_language_code(language_code)?
        } else {
            self.local.prod_find_by_language_code(language_code)?
        };

        match json {
            Some(json) if !json.is_empty() => {
                let model: NativeLocalizationModel = serde_json::from_str(&json).map_err(|e| {
                    ErrorModel::cache(e.to_string()).with_code(error_code::CACHE_EMPTY)
                })?;
                return Ok(model);
            }
            _ => {
                return Err(ErrorModel::cache(message::CONNECTION_REQUIRED)
                    .with_code(error_code::CACHE_EMPTY));
            }
        }
    }
}

fn serialize(model: &NativeLocalizationModel) -> NBResult<String> {
    return serde_json::to_string(model).map_err(|e| ErrorModel::cache(e.to_string()));
}

fn with_code(error: ErrorModel, code: &str) -> ErrorModel {
    if error.error_code.is_some() {
        return error;
    }
    return error.with_code(code);
}
