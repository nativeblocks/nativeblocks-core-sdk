use crate::library::net::network::{GatewayTransport, HttpClient};
use crate::library::result::NBResult;
use std::collections::HashMap;

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

pub(crate) fn percent_encode(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(byte as char);
            }
            _ => encoded.push_str(&format!("%{byte:02X}")),
        }
    }
    return encoded;
}
