//! Tests for `scaffold::client`.

use super::*;
use crate::common::net::Header;

/// A canned-response transport so the feature can be tested without network.
struct FakeHttp {
    body: String,
    last_endpoint: Mutex<Option<String>>,
    is_get: Mutex<Option<bool>>,
}

impl FakeHttp {
    fn new(body: &str) -> Arc<Self> {
        Arc::new(Self {
            body: body.to_string(),
            last_endpoint: Mutex::new(None),
            is_get: Mutex::new(None),
        })
    }
}

#[async_trait]
impl HttpClient for FakeHttp {
    async fn post(&self, endpoint: &str, _headers: &[Header], _body: &str) -> NbResult<String> {
        *self.last_endpoint.lock().unwrap() = Some(endpoint.to_string());
        *self.is_get.lock().unwrap() = Some(false);
        Ok(self.body.clone())
    }

    async fn get(&self, endpoint: &str, _headers: &[Header]) -> NbResult<String> {
        *self.last_endpoint.lock().unwrap() = Some(endpoint.to_string());
        *self.is_get.lock().unwrap() = Some(true);
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

fn request(gateway_type: &str, value: &str) -> ScaffoldRequest {
    ScaffoldRequest {
        gateway: ProjectConfigGateway {
            operation: graphql::GATEWAY_OPERATION.into(),
            gateway_type: gateway_type.into(),
            value: value.into(),
        },
        graphql_endpoint: "https://graphql".into(),
        install_id: "install-1".into(),
    }
}

const SCAFFOLD_JSON: &str = r#"{
    "data": { "scaffold": { "frames": [
        { "id": "1", "name": "Home", "type": "FRAME", "route": "/home",
          "platform": "ANDROID", "routeArguments": [{ "name": "id" }] },
        { "id": "2", "name": "Sheet", "type": "BOTTOM_SHEET", "route": "/sheet" }
    ] } }
}"#;

#[tokio::test]
async fn rest_path_uses_get_on_gateway_value_and_maps_frames() {
    let http = FakeHttp::new(SCAFFOLD_JSON);
    let client = new_client(http.clone(), cloud_env(), SdkConfig::new("TEST"));

    let scaffold = client
        .get_scaffold(request("rest", "https://rest/scaffold"))
        .await
        .unwrap();

    assert_eq!(*http.is_get.lock().unwrap(), Some(true));
    assert_eq!(
        http.last_endpoint.lock().unwrap().as_deref(),
        Some("https://rest/scaffold")
    );
    assert_eq!(scaffold.frames.len(), 2);
    let home = &scaffold.frames[0];
    assert_eq!(home.name.as_deref(), Some("Home"));
    assert_eq!(
        home.frame_type,
        Some(crate::scaffold::model::FrameTypeModel::Frame)
    );
    assert_eq!(
        home.route_arguments.as_ref().unwrap()[0].name.as_deref(),
        Some("id")
    );
    assert_eq!(
        scaffold.frames[1].frame_type,
        Some(crate::scaffold::model::FrameTypeModel::BottomSheet)
    );
}

#[tokio::test]
async fn graphql_path_posts_to_graphql_endpoint() {
    let http = FakeHttp::new(SCAFFOLD_JSON);
    let client = new_client(http.clone(), cloud_env(), SdkConfig::new("TEST"));

    let scaffold = client
        .get_scaffold(request("graphql", "scaffold"))
        .await
        .unwrap();

    assert_eq!(*http.is_get.lock().unwrap(), Some(false));
    assert_eq!(
        http.last_endpoint.lock().unwrap().as_deref(),
        Some("https://graphql")
    );
    assert_eq!(scaffold.frames.len(), 2);
}

#[tokio::test]
async fn graphql_errors_propagate() {
    let http = FakeHttp::new(
        r#"{"errors":[{"message":"nope","extensions":{"classification":"FORBIDDEN"}}]}"#,
    );
    let client = new_client(http, cloud_env(), SdkConfig::new("TEST"));

    let err = client
        .get_scaffold(request("graphql", "scaffold"))
        .await
        .unwrap_err();
    assert_eq!(err.message, "nope");
    assert_eq!(err.error_code.as_deref(), Some("FORBIDDEN"));
}
