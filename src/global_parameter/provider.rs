use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

pub(crate) struct GlobalParameterProvider {
    parameters: Mutex<HashMap<String, String>>,
}

impl GlobalParameterProvider {
    fn new() -> Self {
        return Self { parameters: Mutex::new(HashMap::new()) };
    }

    pub(crate) fn set(&self, parameters: HashMap<String, String>) {
        *self.parameters.lock().unwrap() = parameters;
    }

    pub(crate) fn get(&self) -> HashMap<String, String> {
        return self.parameters.lock().unwrap().clone();
    }
}

fn registry() -> &'static Mutex<HashMap<String, Arc<GlobalParameterProvider>>> {
    static REGISTRY: OnceLock<Mutex<HashMap<String, Arc<GlobalParameterProvider>>>> = OnceLock::new();
    return REGISTRY.get_or_init(|| Mutex::new(HashMap::new()));
}

pub(crate) fn get_or_create(instance_name: &str) -> Arc<GlobalParameterProvider> {
    let mut map = registry().lock().expect("global parameter registry poisoned");
    return map
        .entry(instance_name.to_string())
        .or_insert_with(|| Arc::new(GlobalParameterProvider::new()))
        .clone();
}

pub(crate) fn remove(instance_name: &str) {
    registry()
        .lock()
        .expect("global parameter registry poisoned")
        .remove(instance_name);
}
