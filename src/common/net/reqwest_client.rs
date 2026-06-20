use std::time::Duration;

use async_trait::async_trait;
use reqwest::Client;

use super::{DEFAULT_TIMEOUT_SECS, Header, HttpClient};
use crate::common::result::{ErrorModel, NbResult};

pub struct ReqwestHttpClient {
    client: Client,
}

impl ReqwestHttpClient {
    pub fn new() -> NbResult<Self> {
        Self::with_timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECS))
    }

    pub fn with_timeout(timeout: Duration) -> NbResult<Self> {
        let client = Client::builder()
            .timeout(timeout)
            .connect_timeout(timeout)
            .build()
            .map_err(|e| ErrorModel::network(format!("Failed to build HTTP client: {e}")))?;
        Ok(Self { client })
    }
}

#[async_trait]
impl HttpClient for ReqwestHttpClient {
    async fn post(&self, endpoint: &str, headers: &[Header], body: &str) -> NbResult<String> {
        let mut request = self
            .client
            .post(endpoint)
            .header("Content-Type", "application/json")
            .body(body.to_string());
        for (name, value) in headers {
            request = request.header(name, value);
        }

        let response = request.send().await.map_err(map_reqwest_error)?;
        response.text().await.map_err(map_reqwest_error)
    }

    async fn get(&self, endpoint: &str, headers: &[Header]) -> NbResult<String> {
        let mut request = self.client.get(endpoint);
        for (name, value) in headers {
            request = request.header(name, value);
        }

        let response = request.send().await.map_err(map_reqwest_error)?;
        response.text().await.map_err(map_reqwest_error)
    }
}

fn map_reqwest_error(error: reqwest::Error) -> ErrorModel {
    if error.is_timeout() || error.is_connect() {
        ErrorModel::network("Network timeout or disconnected")
    } else if let Some(status) = error.status() {
        ErrorModel::network(format!("HTTP error: {}", status.as_u16()))
    } else {
        ErrorModel::network(error.to_string())
    }
}
