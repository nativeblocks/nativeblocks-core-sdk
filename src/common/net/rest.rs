use std::collections::HashMap;

use crate::common::result::NBResult;
use crate::common::util::percent_encode;

use super::{GatewayTransport, HttpClient};

pub(crate) const GATEWAY_TYPE_REST: &str = "rest";

pub(crate) struct RestTransport {
    url: String,
    variables: Vec<(String, String)>,
}

impl RestTransport {
    pub(crate) fn new(url: impl Into<String>, variables: Vec<(String, String)>) -> Self {
        return Self {
            url: url.into(),
            variables,
        };
    }

    fn full_url(&self) -> String {
        let mut url = self.url.clone();
        for (key, value) in &self.variables {
            let separator = if url.contains('?') { '&' } else { '?' };
            url.push_str(&format!("{separator}{key}={}", percent_encode(value)));
        }
        return url;
    }
}

#[async_trait::async_trait]
impl GatewayTransport for RestTransport {
    async fn send(
        &self,
        client: &dyn HttpClient,
        headers: HashMap<String, String>,
    ) -> NBResult<String> {
        return Ok(client.get(self.full_url(), headers).await?);
    }
}
