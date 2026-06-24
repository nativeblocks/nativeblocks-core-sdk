use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

/// Per-instance store of global parameters — host-set key/values merged into a
/// frame's variables on load (they override route arguments). Mirrors the old
/// `GlobalParameterDataSource`: an in-memory map, replaced wholesale on set.
pub(super) struct GlobalParameterStore {
    parameters: Mutex<HashMap<String, String>>,
}

impl GlobalParameterStore {
    fn new() -> Self {
        return Self {
            parameters: Mutex::new(HashMap::new()),
        };
    }

    pub(super) fn set(&self, parameters: HashMap<String, String>) {
        *self.parameters.lock().unwrap() = parameters;
    }

    pub(super) fn get(&self) -> HashMap<String, String> {
        return self.parameters.lock().unwrap().clone();
    }
}

fn registry() -> &'static Mutex<HashMap<String, Arc<GlobalParameterStore>>> {
    static REGISTRY: OnceLock<Mutex<HashMap<String, Arc<GlobalParameterStore>>>> = OnceLock::new();
    return REGISTRY.get_or_init(|| Mutex::new(HashMap::new()));
}

pub(super) fn get_or_create(instance_name: &str) -> Arc<GlobalParameterStore> {
    let mut map = registry().lock().expect("global parameter registry poisoned");
    return map
        .entry(instance_name.to_string())
        .or_insert_with(|| Arc::new(GlobalParameterStore::new()))
        .clone();
}
