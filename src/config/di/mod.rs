use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use crate::common::cache::CacheProvider;
use crate::common::config::{NativeblocksEnvironment, SdkConfig};
use crate::common::net::HttpClient;
use crate::config::presenter::client::Client;
use crate::config::data::repository::ProjectConfigRepository;
use crate::config::data::source::ProjectConfigRemoteSource;

fn registry() -> &'static Mutex<HashMap<String, Arc<Client>>> {
    static REGISTRY: OnceLock<Mutex<HashMap<String, Arc<Client>>>> = OnceLock::new();
    return REGISTRY.get_or_init(|| Mutex::new(HashMap::new()));
}

pub(crate) fn get_or_create(
    http: Arc<dyn HttpClient>,
    environment: &NativeblocksEnvironment,
    config: &SdkConfig,
    cache: Arc<dyn CacheProvider>,
) -> Arc<Client> {
    let mut map = registry().lock().expect("config registry poisoned");
    return map
        .entry(environment.instance_name().to_string())
        .or_insert_with(|| build_client(http, environment.clone(), config.clone(), cache))
        .clone();
}

fn build_client(
    http: Arc<dyn HttpClient>,
    environment: NativeblocksEnvironment,
    config: SdkConfig,
    cache: Arc<dyn CacheProvider>,
) -> Arc<Client> {
    let source = ProjectConfigRemoteSource::new(http, environment, config);
    let repository = ProjectConfigRepository::new(source, cache);
    return Arc::new(Client::new(repository));
}
