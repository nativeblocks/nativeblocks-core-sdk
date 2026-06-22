use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use crate::common::cache::CacheProvider;
use crate::common::environment::{NativeblocksEnvironment, SdkConfig};
use crate::common::net::HttpClient;
use crate::config::client::Client;

fn registry() -> &'static Mutex<HashMap<String, Arc<Client>>> {
    static REGISTRY: OnceLock<Mutex<HashMap<String, Arc<Client>>>> = OnceLock::new();
    return REGISTRY.get_or_init(|| Mutex::new(HashMap::new()));
}

pub(crate) fn get_or_create(
    http: Arc<dyn HttpClient>,
    environment: &NativeblocksEnvironment,
    sdk_config: &SdkConfig,
    cache: Arc<dyn CacheProvider>,
) -> Arc<Client> {
    let mut map = registry().lock().expect("config registry poisoned");
    return map
        .entry(environment.instance_name().to_string())
        .or_insert_with(|| {
            Arc::new(Client::new(
                http,
                environment.clone(),
                sdk_config.clone(),
                cache,
            ))
        })
        .clone();
}
