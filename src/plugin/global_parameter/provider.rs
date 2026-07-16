use std::collections::HashMap;
use std::sync::Mutex;

pub(crate) struct GlobalParameterProvider {
    parameters: Mutex<HashMap<String, String>>,
}

impl GlobalParameterProvider {
    pub(crate) fn new() -> Self {
        return Self {
            parameters: Mutex::new(HashMap::new()),
        };
    }

    pub(crate) fn set(&self, parameters: HashMap<String, String>) {
        *self.parameters.lock().unwrap() = parameters;
    }

    pub(crate) fn get(&self) -> HashMap<String, String> {
        return self.parameters.lock().unwrap().clone();
    }
}
