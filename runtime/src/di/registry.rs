use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use crate::library::cache::CacheProvider;
use crate::library::environment::model::{NativeblocksEnvironment, SdkConfig};
use crate::library::net::network::HttpClient;
use crate::library::result::NBResult;

use super::Container;

fn registry() -> &'static Mutex<HashMap<String, Arc<Container>>> {
    static REGISTRY: OnceLock<Mutex<HashMap<String, Arc<Container>>>> = OnceLock::new();
    return REGISTRY.get_or_init(|| Mutex::new(HashMap::new()));
}

pub(crate) fn get_or_create(
    environment: &NativeblocksEnvironment,
    sdk_config: &SdkConfig,
    http: Arc<dyn HttpClient>,
    cache: Arc<dyn CacheProvider>,
) -> NBResult<Arc<Container>> {
    let mut map = registry().lock().expect("di registry poisoned");
    let container = map
        .entry(environment.instance_name().to_string())
        .or_insert_with(|| {
            Arc::new(Container::new(
                environment.clone(),
                sdk_config.clone(),
                http,
                cache,
            ))
        })
        .clone();
    return Ok(container);
}

pub(crate) fn remove(instance_name: &str) -> Option<Arc<Container>> {
    return registry()
        .lock()
        .expect("di registry poisoned")
        .remove(instance_name);
}
