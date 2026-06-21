use std::sync::Arc;

use crate::common::config::{NativeblocksEnvironment, ProjectConfigGateway, SdkConfig};
use crate::common::net::{
    GATEWAY_TYPE_REST, GraphQlRequest, Header, HttpClient, INSTALL_ID_HEADER, decode_envelope,
    execute_graphql, with_headers,
};
use crate::common::result::NBResult;
use crate::localization::data::dto::{
    NativeLocalizationDataDto, NativeLocalizationProductionChecksumDataDto,
};
use crate::localization::data::mapper::localization_to_model;
use crate::localization::graphql;
use crate::localization::model::NativeLocalizationModel;

pub(crate) struct LocalizationRemoteSource {
    http: Arc<dyn HttpClient>,
    environment: NativeblocksEnvironment,
    config: SdkConfig,
}

impl LocalizationRemoteSource {
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

    fn headers(&self, install_id: &str) -> Vec<Header> {
        let mut headers = with_headers(&self.environment, &self.config);
        headers.push((INSTALL_ID_HEADER.to_string(), install_id.to_string()));
        return headers;
    }

    pub(crate) async fn fetch(
        &self,
        gateway: &ProjectConfigGateway,
        graphql_endpoint: &str,
        language_code: &str,
        install_id: &str,
        production: bool,
    ) -> NBResult<NativeLocalizationModel> {
        let headers = self.headers(install_id);

        let dto: NativeLocalizationDataDto = if gateway.gateway_type == GATEWAY_TYPE_REST {
            let url = format!("{}&languageCode={}", gateway.value, language_code);
            let body = self.http.get(&url, &headers).await?;
            decode_envelope::<NativeLocalizationDataDto>(&body)?
        } else {
            let query = if production {
                graphql::LOCALIZATIONS_PRODUCTION_QUERY
            } else {
                graphql::LOCALIZATIONS_QUERY
            };
            let request =
                GraphQlRequest::new(query).with_variables(graphql::variables(language_code));
            execute_graphql::<NativeLocalizationDataDto>(
                self.http.as_ref(),
                graphql_endpoint,
                &headers,
                &request,
            )
            .await?
        };

        let model = if production {
            localization_to_model(dto.localizations_production.as_ref())
        } else {
            localization_to_model(dto.localizations.as_ref())
        };
        return Ok(model);
    }

    pub(crate) async fn fetch_checksum(
        &self,
        gateway: &ProjectConfigGateway,
        graphql_endpoint: &str,
        language_code: &str,
        install_id: &str,
    ) -> NBResult<String> {
        let headers = self.headers(install_id);

        let dto: NativeLocalizationProductionChecksumDataDto =
            if gateway.gateway_type == GATEWAY_TYPE_REST {
                let url = format!("{}&languageCode={}", gateway.value, language_code);
                let body = self.http.get(&url, &headers).await?;
                decode_envelope::<NativeLocalizationProductionChecksumDataDto>(&body)?
            } else {
                let request = GraphQlRequest::new(graphql::PRODUCTION_CHECKSUM_QUERY)
                    .with_variables(graphql::variables(language_code));
                execute_graphql::<NativeLocalizationProductionChecksumDataDto>(
                    self.http.as_ref(),
                    graphql_endpoint,
                    &headers,
                    &request,
                )
                .await?
            };

        return Ok(dto
            .localization_production_checksum
            .and_then(|c| c.checksum)
            .unwrap_or_default());
    }
}
