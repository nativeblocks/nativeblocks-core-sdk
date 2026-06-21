//! Tests for `experiment::client`.

use super::*;
use crate::common::cache::sqlite::SqliteCacheProvider;
use crate::common::net::Header;

struct FakeHttp {
    body: String,
    last_endpoint: Mutex<Option<String>>,
    is_get: Mutex<Option<bool>>,
    calls: Mutex<u32>,
}

impl FakeHttp {
    fn new(body: &str) -> Arc<Self> {
        Arc::new(Self {
            body: body.to_string(),
            last_endpoint: Mutex::new(None),
            is_get: Mutex::new(None),
            calls: Mutex::new(0),
        })
    }
}

#[async_trait]
impl HttpClient for FakeHttp {
    async fn post(&self, endpoint: &str, _headers: &[Header], _body: &str) -> NbResult<String> {
        *self.last_endpoint.lock().unwrap() = Some(endpoint.to_string());
        *self.is_get.lock().unwrap() = Some(false);
        *self.calls.lock().unwrap() += 1;
        Ok(self.body.clone())
    }

    async fn get(&self, endpoint: &str, _headers: &[Header]) -> NbResult<String> {
        *self.last_endpoint.lock().unwrap() = Some(endpoint.to_string());
        *self.is_get.lock().unwrap() = Some(true);
        *self.calls.lock().unwrap() += 1;
        Ok(self.body.clone())
    }
}

fn cloud_env() -> NativeblocksEnvironment {
    NativeblocksEnvironment::Cloud {
        instance_name: "test".into(),
        endpoint: "https://config".into(),
        api_key: "key".into(),
        development_mode: false,
    }
}

fn community_env() -> NativeblocksEnvironment {
    NativeblocksEnvironment::Community {
        instance_name: "test".into(),
        frames_data: HashMap::new(),
    }
}

fn cache() -> Arc<dyn CacheProvider> {
    Arc::new(SqliteCacheProvider::in_memory().unwrap())
}

fn request(gateway_type: &str, value: &str) -> ExperimentRequest {
    ExperimentRequest {
        gateway: ProjectConfigGateway {
            operation: graphql::GATEWAY_OPERATION.into(),
            gateway_type: gateway_type.into(),
            value: value.into(),
        },
        graphql_endpoint: "https://graphql".into(),
        install_id: "install-1".into(),
        key: "exp-1".into(),
        parameters: HashMap::new(),
        cache_ttl_millis: 60_000,
    }
}

const EXPERIMENT_JSON: &str = r#"{
    "data": { "experimentValue": { "key": "exp-1", "value": "B", "variableType": "STRING" } }
}"#;

#[tokio::test]
async fn graphql_path_maps_value_and_type() {
    let http = FakeHttp::new(EXPERIMENT_JSON);
    let client = new_client(http.clone(), cloud_env(), SdkConfig::new("TEST"), cache());

    let model = client
        .get_experiment(request("graphql", "experimentValue"))
        .await
        .unwrap();

    assert_eq!(*http.is_get.lock().unwrap(), Some(false));
    assert_eq!(
        http.last_endpoint.lock().unwrap().as_deref(),
        Some("https://graphql")
    );
    assert_eq!(model.value, "B");
    assert_eq!(model.variable_type, "STRING");
}

#[tokio::test]
async fn rest_path_gets_with_key_query_param() {
    let http = FakeHttp::new(EXPERIMENT_JSON);
    let client = new_client(http.clone(), cloud_env(), SdkConfig::new("TEST"), cache());

    let model = client
        .get_experiment(request("rest", "https://rest/experiment"))
        .await
        .unwrap();

    assert_eq!(*http.is_get.lock().unwrap(), Some(true));
    assert_eq!(
        http.last_endpoint.lock().unwrap().as_deref(),
        Some("https://rest/experiment?key=exp-1")
    );
    assert_eq!(model.value, "B");
}

#[tokio::test]
async fn second_call_is_served_from_cache() {
    let http = FakeHttp::new(EXPERIMENT_JSON);
    let client = new_client(http.clone(), cloud_env(), SdkConfig::new("TEST"), cache());

    let _ = client
        .get_experiment(request("graphql", "experimentValue"))
        .await
        .unwrap();
    let model = client
        .get_experiment(request("graphql", "experimentValue"))
        .await
        .unwrap();

    assert_eq!(*http.calls.lock().unwrap(), 1);
    assert_eq!(model.value, "B");
    assert_eq!(model.variable_type, "STRING");
}

#[tokio::test]
async fn community_environment_is_not_supported() {
    let http = FakeHttp::new(EXPERIMENT_JSON);
    let client = new_client(http.clone(), community_env(), SdkConfig::new("TEST"), cache());

    let error = client
        .get_experiment(request("graphql", "experimentValue"))
        .await
        .unwrap_err();

    assert_eq!(error.error_type, crate::common::result::ErrorType::Support);
    assert_eq!(*http.calls.lock().unwrap(), 0);
}
