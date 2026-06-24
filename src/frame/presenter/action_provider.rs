use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use crate::frame::presenter::action_props::ActionProps;

#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait NativeActionHandler: Send + Sync {
    async fn handle(&self, props: Arc<ActionProps>);
}

/// Invoked when a trigger's `keyType` has no registered handler. The host decides
/// what to do (e.g. surface a "not available in this version" message).
#[uniffi::export(with_foreign)]
pub trait NativeActionFallback: Send + Sync {
    fn handle(&self, key_type: String, name: String);
}

pub(super) struct ActionProvider {
    handlers: Mutex<HashMap<String, Arc<dyn NativeActionHandler>>>,
    fallback: Mutex<Option<Arc<dyn NativeActionFallback>>>,
}

impl ActionProvider {
    pub(super) fn new() -> Self {
        return Self {
            handlers: Mutex::new(HashMap::new()),
            fallback: Mutex::new(None),
        };
    }

    pub(super) fn provide(&self, key_type: String, handler: Arc<dyn NativeActionHandler>) {
        self.handlers.lock().unwrap().insert(key_type, handler);
    }

    pub(super) fn find(&self, key_type: &str) -> Option<Arc<dyn NativeActionHandler>> {
        return self.handlers.lock().unwrap().get(key_type).cloned();
    }

    pub(super) fn provide_fallback(&self, fallback: Arc<dyn NativeActionFallback>) {
        *self.fallback.lock().unwrap() = Some(fallback);
    }

    pub(super) fn fallback(&self) -> Option<Arc<dyn NativeActionFallback>> {
        return self.fallback.lock().unwrap().clone();
    }
}

fn registry() -> &'static Mutex<HashMap<String, Arc<ActionProvider>>> {
    static REGISTRY: OnceLock<Mutex<HashMap<String, Arc<ActionProvider>>>> = OnceLock::new();
    return REGISTRY.get_or_init(|| Mutex::new(HashMap::new()));
}

pub(super) fn get_or_create(instance_name: &str) -> Arc<ActionProvider> {
    let mut map = registry().lock().expect("action registry poisoned");
    return map
        .entry(instance_name.to_string())
        .or_insert_with(|| Arc::new(ActionProvider::new()))
        .clone();
}

#[uniffi::export]
pub fn provide_action(
    instance_name: String,
    key_type: String,
    handler: Arc<dyn NativeActionHandler>,
) {
    get_or_create(&instance_name).provide(key_type, handler);
}

#[uniffi::export]
pub fn provide_action_fallback(instance_name: String, fallback: Arc<dyn NativeActionFallback>) {
    get_or_create(&instance_name).provide_fallback(fallback);
}
