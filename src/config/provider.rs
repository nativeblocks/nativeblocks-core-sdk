use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use crate::common::cache::{CacheProvider, run_migrations};
use crate::common::environment::{NativeblocksEnvironment, SdkConfig};
use crate::common::net::HttpClient;
use crate::common::result::NBResult;
use crate::config::client::Client;
use crate::config::key::INSTALL_ID_KEY;

fn registry() -> &'static Mutex<HashMap<String, Arc<Client>>> {
    static REGISTRY: OnceLock<Mutex<HashMap<String, Arc<Client>>>> = OnceLock::new();
    return REGISTRY.get_or_init(|| Mutex::new(HashMap::new()));
}

pub(crate) fn get_or_create(
    http: Arc<dyn HttpClient>,
    environment: &NativeblocksEnvironment,
    sdk_config: &SdkConfig,
    cache: Arc<dyn CacheProvider>,
) -> NBResult<Arc<Client>> {
    let mut map = registry().lock().expect("config registry poisoned");
    if let Some(client) = map.get(environment.instance_name()) {
        return Ok(client.clone());
    }
    run_migrations(cache.as_ref(), &[INSTALL_ID_KEY])?;
    let client = Arc::new(Client::new(
        http,
        environment.clone(),
        sdk_config.clone(),
        cache,
    ));
    map.insert(environment.instance_name().to_string(), client.clone());
    return Ok(client);
}
