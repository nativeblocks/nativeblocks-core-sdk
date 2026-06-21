//! Tests for `config::client`.

use super::*;
use crate::common::net::Header;
use std::sync::atomic::{AtomicUsize, Ordering};

struct FakeHttp {
    body: String,
    last_get: Mutex<Option<String>>,
    get_calls: AtomicUsize,
}

impl FakeHttp {
    fn new(body: &str) -> Arc<Self> {
        Arc::new(Self {
            body: body.to_string(),
            last_get: Mutex::new(None),
            get_calls: AtomicUsize::new(0),
        })
    }
}

#[async_trait]
impl HttpClient for FakeHttp {
    async fn post(&self, _endpoint: &str, _headers: &[Header], _body: &str) -> NbResult<String> {
        unreachable!("config only fetches via GET")
    }

    async fn get(&self, endpoint: &str, _headers: &[Header]) -> NbResult<String> {
        self.get_calls.fetch_add(1, Ordering::SeqCst);
        *self.last_get.lock().unwrap() = Some(endpoint.to_string());
        Ok(self.body.clone())
    }
}

fn cloud_env() -> NativeblocksEnvironment {
    NativeblocksEnvironment::Cloud {
        instance_name: "test".into(),
        endpoint: "https://api.nativeblocks.io/gateway/init".into(),
        api_key: "key".into(),
        development_mode: false,
    }
}

const CONFIG_JSON: &str = r#"{
    "data": { "projectConfig": {
        "gateway": "graphql",
        "endpoint": "https://api.nativeblocks.io/graphql",
        "endpoints": [
            { "operation": "frame", "type": "rest", "value": "https://api.nativeblocks.io/rest/frame" }
        ]
    } }
}"#;

#[tokio::test]
async fn fetches_init_endpoint_and_maps_config() {
    let http = FakeHttp::new(CONFIG_JSON);
    let client = new_client(http.clone(), cloud_env(), SdkConfig::new("TEST"));

    let config = client.project_config("install-1").await.unwrap();

    assert_eq!(
        http.last_get.lock().unwrap().as_deref(),
        Some("https://api.nativeblocks.io/gateway/init")
    );
    assert_eq!(config.endpoint, "https://api.nativeblocks.io/graphql");
    assert_eq!(config.endpoints.len(), 1);
}

#[tokio::test]
async fn caches_after_first_fetch() {
    let http = FakeHttp::new(CONFIG_JSON);
    let client = new_client(http.clone(), cloud_env(), SdkConfig::new("TEST"));

    client.project_config("install-1").await.unwrap();
    client.project_config("install-1").await.unwrap();

    assert_eq!(http.get_calls.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn gateway_for_uses_config_entry_then_graphql_fallback() {
    let http = FakeHttp::new(CONFIG_JSON);
    let client = new_client(http, cloud_env(), SdkConfig::new("TEST"));
    let config = client.project_config("install-1").await.unwrap();

    let frame = gateway_for(&config, "frame");
    assert_eq!(frame.gateway_type, "rest");
    assert_eq!(frame.value, "https://api.nativeblocks.io/rest/frame");

    let scaffold = gateway_for(&config, "scaffold");
    assert_eq!(scaffold.gateway_type, "graphql");
    assert_eq!(scaffold.value, "scaffold");
}
