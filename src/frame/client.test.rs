use std::collections::VecDeque;

use super::*;
use crate::common::net::Header;
use crate::frame::data::source::sqlite::SqliteFrameDatabase;
use crate::frame::model::{NativeActionTriggerPropertyModel, NativeActionTriggerThen};

struct FakeHttp {
    responses: Mutex<VecDeque<String>>,
    calls: Mutex<usize>,
}

impl FakeHttp {
    fn new(responses: &[&str]) -> Arc<Self> {
        Arc::new(Self {
            responses: Mutex::new(responses.iter().map(|s| s.to_string()).collect()),
            calls: Mutex::new(0),
        })
    }

    fn next(&self) -> NbResult<String> {
        *self.calls.lock().unwrap() += 1;
        self.responses
            .lock()
            .unwrap()
            .pop_front()
            .ok_or_else(|| crate::common::result::ErrorModel::network("no canned response left"))
    }

    fn call_count(&self) -> usize {
        *self.calls.lock().unwrap()
    }
}

#[async_trait]
impl HttpClient for FakeHttp {
    async fn post(&self, _endpoint: &str, _headers: &[Header], _body: &str) -> NbResult<String> {
        self.next()
    }
    async fn get(&self, _endpoint: &str, _headers: &[Header]) -> NbResult<String> {
        self.next()
    }
}

struct FakeLocalization {
    map: HashMap<String, String>,
}

#[async_trait]
impl localization::Client for FakeLocalization {
    async fn sync_localization(
        &self,
        _request: localization::LocalizationSyncRequest,
    ) -> NbResult<()> {
        Ok(())
    }
    async fn get_localization(&self, _language_code: String) -> NbResult<()> {
        Ok(())
    }
    fn set_language_code(&self, _language_code: String) {}
    fn get_language_code(&self) -> watch::Receiver<Option<String>> {
        watch::channel(None).1
    }
    fn translate(&self, key: String) -> Option<String> {
        self.map.get(&key).cloned()
    }
}

fn gateway(operation: &str, gateway_type: &str, value: &str) -> ProjectConfigGateway {
    ProjectConfigGateway {
        operation: operation.into(),
        gateway_type: gateway_type.into(),
        value: value.into(),
    }
}

fn sync_request(route: &str) -> FrameSyncRequest {
    FrameSyncRequest {
        endpoint_frame: gateway("frame", "graphql", "frame"),
        endpoint_frame_production: gateway("frameProduction", "graphql", "frameProduction"),
        endpoint_frame_production_checksum: gateway(
            "frameProductionChecksum",
            "graphql",
            "frameProductionChecksum",
        ),
        graphql_endpoint: "https://graphql".into(),
        install_id: "install-1".into(),
        route: route.into(),
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

fn engine(
    http: Arc<FakeHttp>,
    environment: NativeblocksEnvironment,
    localization_map: HashMap<String, String>,
) -> Arc<dyn Client> {
    let source: Arc<dyn FrameLocalSource> = SqliteFrameDatabase::in_memory().unwrap();
    let localization: Arc<dyn localization::Client> = Arc::new(FakeLocalization {
        map: localization_map,
    });
    new_client(http, environment, SdkConfig::default(), source, localization)
}

const FRAME_JSON: &str = r#"{
    "data": {
        "frame": {
            "variables": [{ "key": "name", "value": "Bob", "type": "STRING" }],
            "blocks": [{
                "id": "b1", "parentId": "", "integrationVersion": 1, "slot": "",
                "keyType": "ROOT", "key": "root", "visibilityKey": "visible", "position": 0,
                "properties": [], "data": [], "slots": []
            }],
            "actions": [{
                "id": "a1", "key": "root", "event": "onAppear",
                "triggers": [{
                    "id": "t1", "parentId": "", "integrationVersion": 1, "name": "log",
                    "keyType": "LOG", "then": "NEXT", "properties": [], "data": []
                }]
            }]
        }
    }
}"#;

const PROD_FRAME_JSON: &str = r#"{
    "data": {
        "frameProduction": {
            "checksum": "abc",
            "variables": [{ "key": "name", "value": "Bob", "type": "STRING" }],
            "blocks": [{
                "id": "b1", "parentId": "", "integrationVersion": 1, "slot": "",
                "keyType": "ROOT", "key": "root", "visibilityKey": "visible", "position": 0,
                "properties": [], "data": [], "slots": []
            }],
            "actions": []
        }
    }
}"#;

const CHECKSUM_MATCH_JSON: &str = r#"{ "data": { "frameProductionChecksum": { "checksum": "abc" } } }"#;

#[tokio::test]
async fn setup_with_empty_cache_reports_loading() {
    let http = FakeHttp::new(&[]);
    let engine = engine(http, cloud_env(true), HashMap::new());

    engine
        .setup_frame("home".into(), HashMap::new())
        .await;

    let state = engine.native_frame_state();
    assert!(state.is_loading);
    assert!(!state.is_success);
}

#[tokio::test]
async fn dev_sync_populates_observable_state() {
    let http = FakeHttp::new(&[FRAME_JSON]);
    let engine = engine(http, cloud_env(true), HashMap::new());

    let mut args = HashMap::new();
    args.insert("id".to_string(), "42".to_string());
    engine.setup_frame("home".into(), args).await;

    engine.sync_frame(sync_request("home")).await.unwrap();

    let state = engine.native_frame_state();
    assert!(state.is_success, "frame should be successful after sync");
    assert_eq!(engine.frame_update_generation(), 1);

    let variables = engine.variables_state();
    assert_eq!(variables.get("name").map(|v| v.value.as_str()), Some("Bob"));
    assert_eq!(variables.get("id").map(|v| v.value.as_str()), Some("42"));

    let blocks = engine.blocks_state();
    assert!(blocks.contains_key("root"));

    let actions = engine.action_state();
    assert!(actions.contains_key("root"));
}

#[tokio::test]
async fn observers_receive_state_snapshots() {
    let http = FakeHttp::new(&[FRAME_JSON]);
    let engine = engine(http, cloud_env(true), HashMap::new());

    let mut blocks_rx = engine.observe_blocks();

    engine.setup_frame("home".into(), HashMap::new()).await;
    engine.sync_frame(sync_request("home")).await.unwrap();

    assert!(blocks_rx.has_changed().unwrap());
    assert!(blocks_rx.borrow_and_update().contains_key("root"));
}

#[tokio::test]
async fn production_sync_is_checksum_gated() {
    let http = FakeHttp::new(&[PROD_FRAME_JSON, CHECKSUM_MATCH_JSON]);
    let counter = http.clone();
    let engine = engine(http, cloud_env(false), HashMap::new());

    engine.setup_frame("home".into(), HashMap::new()).await;

    // First sync: no cached production frame, fetch and store it.
    engine.sync_frame(sync_request("home")).await.unwrap();
    // Second sync: cached frame exists, checksum matches, so no extra fetch.
    engine.sync_frame(sync_request("home")).await.unwrap();

    assert_eq!(counter.call_count(), 2, "checksum match must skip re-fetch");
    assert!(engine.native_frame_state().is_success);
}

#[tokio::test]
async fn localize_delegates_to_localization_feature() {
    let http = FakeHttp::new(&[]);
    let mut map = HashMap::new();
    map.insert("greeting".to_string(), "Hello".to_string());
    let engine = engine(http, cloud_env(true), map);

    assert_eq!(engine.localize("greeting".into()), Some("Hello".to_string()));
    assert_eq!(engine.localize("missing".into()), None);
}

#[tokio::test]
async fn handle_variable_updates_state_and_stream() {
    let http = FakeHttp::new(&[]);
    let engine = engine(http, cloud_env(true), HashMap::new());

    engine.handle_variable(NativeVariableModel::new("count", "1", "INT"), false);
    assert_eq!(
        engine.variables_state().get("count").map(|v| v.value.as_str()),
        Some("1")
    );
}

struct FakeActionHandler {
    then: NativeActionTriggerThen,
    variable: Option<(String, String)>,
}

#[async_trait]
impl NativeActionHandler for FakeActionHandler {
    async fn handle(&self, _context: ActionContext) -> ActionResult {
        let variable_changes = self
            .variable
            .as_ref()
            .map(|(k, v)| vec![NativeVariableModel::new(k, v, "STRING")])
            .unwrap_or_default();
        ActionResult {
            then: self.then,
            variable_changes,
            block_changes: Vec::new(),
        }
    }
}

fn handler(then: NativeActionTriggerThen, key: &str, value: &str) -> Arc<dyn NativeActionHandler> {
    Arc::new(FakeActionHandler {
        then,
        variable: Some((key.to_string(), value.to_string())),
    })
}

fn trigger(
    id: &str,
    parent: &str,
    key_type: &str,
    then: NativeActionTriggerThen,
) -> NativeActionTriggerModel {
    NativeActionTriggerModel {
        name: key_type.into(),
        id: id.into(),
        parent_id: parent.into(),
        version: 1,
        key_type: key_type.into(),
        then,
        properties: HashMap::new(),
        data: HashMap::new(),
    }
}

fn action(triggers: Vec<NativeActionTriggerModel>) -> NativeActionModel {
    NativeActionModel {
        id: "act".into(),
        key: "btn".into(),
        event: "onClick".into(),
        triggers,
    }
}

#[tokio::test]
async fn trigger_graph_follows_then_branch() {
    let engine = engine(FakeHttp::new(&[]), cloud_env(true), HashMap::new());
    engine.register_action_handler("SET_A".into(), handler(NativeActionTriggerThen::Next, "a", "1"));
    engine.register_action_handler("SET_B".into(), handler(NativeActionTriggerThen::Success, "b", "2"));
    engine.register_action_handler("SET_C".into(), handler(NativeActionTriggerThen::Next, "c", "3"));

    let act = action(vec![
        trigger("t1", "", "SET_A", NativeActionTriggerThen::Next),
        // child on SUCCESS branch must NOT run (t1 returned NEXT)
        trigger("t2", "t1", "SET_B", NativeActionTriggerThen::Success),
        // child on NEXT branch must run
        trigger("t3", "t1", "SET_C", NativeActionTriggerThen::Next),
    ]);

    engine.handle_action(0, Some(act), "onClick".into()).await;

    let vars = engine.variables_state();
    assert_eq!(vars.get("a").map(|v| v.value.as_str()), Some("1"));
    assert_eq!(vars.get("c").map(|v| v.value.as_str()), Some("3"));
    assert!(!vars.contains_key("b"), "SUCCESS branch must not run after NEXT");
}

#[tokio::test]
async fn event_mismatch_runs_nothing() {
    let engine = engine(FakeHttp::new(&[]), cloud_env(true), HashMap::new());
    engine.register_action_handler("SET_A".into(), handler(NativeActionTriggerThen::Next, "a", "1"));
    let act = action(vec![trigger("t1", "", "SET_A", NativeActionTriggerThen::Next)]);

    engine.handle_action(0, Some(act), "onAppear".into()).await;

    assert!(engine.variables_state().is_empty());
}

#[tokio::test]
async fn unregistered_action_falls_back_without_panic() {
    let engine = engine(FakeHttp::new(&[]), cloud_env(true), HashMap::new());
    let act = action(vec![trigger("t1", "", "UNKNOWN", NativeActionTriggerThen::Next)]);

    engine.handle_action(0, Some(act), "onClick".into()).await;

    assert!(engine.variables_state().is_empty());
}

#[cfg(feature = "script-quickjs")]
#[tokio::test]
async fn script_action_mutates_variable_in_core() {
    let engine = engine(FakeHttp::new(&[]), cloud_env(true), HashMap::new());
    engine.handle_variable(NativeVariableModel::new("count", "1", "INT"), false);

    let mut properties = HashMap::new();
    properties.insert(
        "script".to_string(),
        NativeActionTriggerPropertyModel {
            key: "script".into(),
            value: "updateVariable('count', Number(getVariable('count')) + 1)".into(),
            value_type: "STRING".into(),
        },
    );
    let mut script_trigger = trigger(
        "t1",
        "",
        crate::frame::action::SCRIPT_KEY_TYPE,
        NativeActionTriggerThen::Next,
    );
    script_trigger.properties = properties;

    engine.handle_action(0, Some(action(vec![script_trigger])), "onClick".into()).await;

    assert_eq!(
        engine.variables_state().get("count").map(|v| v.value.as_str()),
        Some("2")
    );
}

#[test]
fn frame_json_round_trips_legacy_structure() {
    // A frameJson string as serialized by the Kotlin engine must decode without
    // migration: camelCase keys, `type` field, omitted nulls.
    let legacy = r#"{"variables":{"name":{"key":"name","value":"Bob","type":"STRING"}},"blocks":{"root":{"id":"b1","parentId":"","version":1,"slot":"","keyType":"ROOT","key":"root","visibility":"visible","position":0,"data":{},"properties":{},"slots":{}}},"actions":{}}"#;
    let model: NativeFrameModel = serde_json::from_str(legacy).unwrap();
    let block = &model.blocks.as_ref().unwrap()["root"];
    assert_eq!(block.key_type, "ROOT");
    assert_eq!(block.version, 1);
    let variable = &model.variables.as_ref().unwrap()["name"];
    assert_eq!(variable.value_type, "STRING");
}
