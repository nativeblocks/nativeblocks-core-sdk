use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use crate::frame::presenter::block_props::BlockProps;

#[uniffi::export(with_foreign)]
pub trait NativeBlockHandler: Send + Sync {
    fn handle(&self, key_type: String, props: Arc<BlockProps>);
}

#[uniffi::export(with_foreign)]
pub trait NativeBlockFallback: Send + Sync {
    fn handle(&self, key_type: String, key: String);
}

#[uniffi::export(with_foreign)]
pub trait BlockFinder: Send + Sync {
    fn find_handler(&self, key_type: String) -> Option<Arc<dyn NativeBlockHandler>>;
    fn fallback(&self, key_type: String, key: String);
}

pub(super) struct BlockProvider {
    handlers: Mutex<HashMap<String, Arc<dyn NativeBlockHandler>>>,
    fallback: Mutex<Option<Arc<dyn NativeBlockFallback>>>,
}

impl BlockProvider {
    pub(super) fn new() -> Self {
        return Self {
            handlers: Mutex::new(HashMap::new()),
            fallback: Mutex::new(None),
        };
    }

    pub(super) fn provide(&self, key_type: String, handler: Arc<dyn NativeBlockHandler>) {
        self.handlers.lock().unwrap().insert(key_type, handler);
    }

    pub(super) fn find(&self, key_type: &str) -> Option<Arc<dyn NativeBlockHandler>> {
        return self.handlers.lock().unwrap().get(key_type).cloned();
    }

    pub(super) fn provide_fallback(&self, fallback: Arc<dyn NativeBlockFallback>) {
        *self.fallback.lock().unwrap() = Some(fallback);
    }

    pub(super) fn fallback(&self) -> Option<Arc<dyn NativeBlockFallback>> {
        return self.fallback.lock().unwrap().clone();
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

pub(super) fn remove(instance_name: &str) {
    registry()
        .lock()
        .expect("block registry poisoned")
        .remove(instance_name);
}

#[uniffi::export]
pub fn provide_block(instance_name: String, key_type: String, handler: Arc<dyn NativeBlockHandler>) {
    get_or_create(&instance_name).provide(key_type, handler);
}

#[uniffi::export]
pub fn provide_block_fallback(instance_name: String, fallback: Arc<dyn NativeBlockFallback>) {
    get_or_create(&instance_name).provide_fallback(fallback);
}

#[uniffi::export]
pub fn block_handler(instance_name: String, key_type: String) -> Option<Arc<dyn NativeBlockHandler>> {
    return get_or_create(&instance_name).find(&key_type);
}

#[uniffi::export]
pub fn block_fallback(instance_name: String) -> Option<Arc<dyn NativeBlockFallback>> {
    return get_or_create(&instance_name).fallback();
}
