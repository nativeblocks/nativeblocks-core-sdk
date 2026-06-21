use std::collections::{HashMap, VecDeque};

use super::*;
use crate::common::net::Header;
use crate::common::result::ErrorType;
use crate::localization::data::source::sqlite::SqliteLocalizationDatabase;

struct FakeHttp {
    responses: Mutex<VecDeque<String>>,
}

impl FakeHttp {
    fn new(responses: &[&str]) -> Arc<Self> {
        Arc::new(Self {
            responses: Mutex::new(responses.iter().map(|s| s.to_string()).collect()),
        })
    }
}

#[async_trait]
impl HttpClient for FakeHttp {
    async fn post(&self, _endpoint: &str, _headers: &[Header], _body: &str) -> NbResult<String> {
        self.responses
            .lock()
            .unwrap()
            .pop_front()
            .ok_or_else(|| ErrorModel::network("no canned response left"))
    }
    async fn get(&self, _endpoint: &str, _headers: &[Header]) -> NbResult<String> {
        self.post("", &[], "").await
    }
}

fn gateway(operation: &str) -> ProjectConfigGateway {
    ProjectConfigGateway {
        operation: operation.into(),
        gateway_type: "graphql".into(),
        value: operation.into(),
    }
}

fn sync_request(language_code: &str) -> LocalizationSyncRequest {
    LocalizationSyncRequest {
        endpoint_localization: gateway("localizations"),
        endpoint_localization_production: gateway("localizationsProduction"),
        endpoint_localization_production_checksum: gateway("localizationProductionChecksum"),
        graphql_endpoint: "https://graphql".into(),
        language_code: language_code.into(),
        install_id: "install-1".into(),
    }
}

fn cloud_env(development_mode: bool) -> NativeblocksEnvironment {
    NativeblocksEnvironment::Cloud {
        instance_name: "test".into(),
        endpoint: "https://config".into(),
        api_key: "key".into(),
        development_mode,
    }
}

fn community_env() -> NativeblocksEnvironment {
    NativeblocksEnvironment::Community {
        instance_name: "test".into(),
        frames_data: HashMap::new(),
    }
}

fn client(http: Arc<FakeHttp>, environment: NativeblocksEnvironment) -> Arc<dyn Client> {
    let source: Arc<dyn LocalizationLocalSource> = SqliteLocalizationDatabase::in_memory().unwrap();
    new_client(http, environment, SdkConfig::default(), source)
}

const LOCALIZATION_JSON: &str = r#"{
    "data": {
        "localizations": {
            "checksum": "l1",
            "localizations": [{ "key": "hi", "value": "Hi" }]
        }
    }
}"#;

#[tokio::test]
async fn dev_sync_then_translate() {
    let http = FakeHttp::new(&[LOCALIZATION_JSON]);
    let client = client(http, cloud_env(true));

    client.sync_localization(sync_request("en")).await.unwrap();

    assert_eq!(client.translate("hi".into()), Some("Hi".to_string()));
    assert_eq!(client.translate("bye".into()), None);
}

#[tokio::test]
async fn set_language_code_emits_on_stream() {
    let http = FakeHttp::new(&[]);
    let client = client(http, cloud_env(true));

    let mut rx = client.get_language_code();
    assert!(rx.borrow().is_none());

    client.set_language_code("fa".into());
    assert!(rx.has_changed().unwrap());
    assert_eq!(rx.borrow_and_update().as_deref(), Some("fa"));
}

#[tokio::test]
async fn get_localization_from_empty_cache_errors() {
    let http = FakeHttp::new(&[]);
    let client = client(http, cloud_env(true));

    let result = client.get_localization("en".into()).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn community_sync_requires_cloud() {
    let http = FakeHttp::new(&[]);
    let client = client(http, community_env());

    let error = client.sync_localization(sync_request("en")).await.unwrap_err();
    assert_eq!(error.error_type, ErrorType::Support);
}
