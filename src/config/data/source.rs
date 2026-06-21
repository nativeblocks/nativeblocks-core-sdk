use std::sync::Arc;

use crate::common::config::{NativeblocksEnvironment, SdkConfig};
use crate::common::net::{HttpClient, INSTALL_ID_HEADER, decode_envelope, with_headers};
use crate::common::result::{ErrorModel, NBResult};
use crate::config::data::dto::ProjectConfigDataDto;
use crate::config::key::{error_code, message};

pub(crate) struct ProjectConfigRemoteSource {
    http: Arc<dyn HttpClient>,
    environment: NativeblocksEnvironment,
    config: SdkConfig,
}

impl ProjectConfigRemoteSource {
    pub(crate) fn new(
        http: Arc<dyn HttpClient>,
        environment: NativeblocksEnvironment,
        config: SdkConfig,
    ) -> Self {
        return Self {
            http,
            environment,
            config,
        };
    }

    pub(crate) async fn fetch(&self, install_id: &str) -> NBResult<ProjectConfigDataDto> {
        let endpoint = self.environment.endpoint().ok_or_else(|| {
            ErrorModel::support(message::CLOUD_ONLY).with_code(error_code::PROJECT_CONFIG)
        })?;
        let mut headers = with_headers(&self.environment, &self.config);
        headers.push((INSTALL_ID_HEADER.to_string(), install_id.to_string()));
        let body = self.http.get(endpoint, &headers).await?;
        return decode_envelope::<ProjectConfigDataDto>(&body);
    }
}
