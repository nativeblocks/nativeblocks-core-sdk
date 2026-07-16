use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use crate::common::environment::model::{NativeblocksEnvironment, SdkConfig};
use crate::common::result::NBResult;

use super::Container;

fn registry() -> &'static Mutex<HashMap<String, Arc<Container>>> {
    static REGISTRY: OnceLock<Mutex<HashMap<String, Arc<Container>>>> = OnceLock::new();
    return REGISTRY.get_or_init(|| Mutex::new(HashMap::new()));
}

pub(crate) fn get_or_create(
    environment: &NativeblocksEnvironment,
    sdk_config: &SdkConfig,
) -> NBResult<Arc<Container>> {
    environment.validate()?;
    let mut map = registry().lock().expect("di registry poisoned");
    let container = map
        .entry(environment.instance_name().to_string())
        .or_insert_with(|| Arc::new(Container::new(environment.clone(), sdk_config.clone())))
        .clone();
    return Ok(container);
}

pub(crate) fn remove(instance_name: &str) {
    registry()
        .lock()
        .expect("di registry poisoned")
        .remove(instance_name);
}
