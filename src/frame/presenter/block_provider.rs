use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use crate::frame::presenter::block_props::BlockProps;

#[uniffi::export(with_foreign)]
pub trait NativeBlockHandler: Send + Sync {
    fn handle(&self, key_type: String, props: Arc<BlockProps>);
}

pub(super) struct BlockProvider {
    handlers: Mutex<HashMap<String, Arc<dyn NativeBlockHandler>>>,
}

impl BlockProvider {
    pub(super) fn new() -> Self {
        return Self {
            handlers: Mutex::new(HashMap::new()),
        };
    }

    pub(super) fn provide(&self, key_type: String, handler: Arc<dyn NativeBlockHandler>) {
        self.handlers.lock().unwrap().insert(key_type, handler);
    }

    #[allow(dead_code)]
    pub(super) fn find(&self, key_type: &str) -> Option<Arc<dyn NativeBlockHandler>> {
        return self.handlers.lock().unwrap().get(key_type).cloned();
    }
}

fn registry() -> &'static Mutex<HashMap<String, Arc<BlockProvider>>> {
    static REGISTRY: OnceLock<Mutex<HashMap<String, Arc<BlockProvider>>>> = OnceLock::new();
    return REGISTRY.get_or_init(|| Mutex::new(HashMap::new()));
}

pub(super) fn get_or_create(instance_name: &str) -> Arc<BlockProvider> {
    let mut map = registry().lock().expect("block registry poisoned");
    return map
        .entry(instance_name.to_string())
        .or_insert_with(|| Arc::new(BlockProvider::new()))
        .clone();
}

#[uniffi::export]
pub fn provide_block(
    instance_name: String,
    key_type: String,
    handler: Arc<dyn NativeBlockHandler>,
) {
    get_or_create(&instance_name).provide(key_type, handler);
}
