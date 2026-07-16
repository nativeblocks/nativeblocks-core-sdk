mod client;
mod dto;
mod key;
mod model;
mod repository;

use crate::library::cache::CacheProvider;
use crate::library::environment::model::{NativeblocksEnvironment, SdkConfig};
use crate::library::net::network::HttpClient;
pub(crate) use client::Client;
pub(crate) use model::ProjectConfigGatewayModel;
use std::sync::Arc;

pub(crate) fn build_client(
    environment: NativeblocksEnvironment,
    sdk_config: SdkConfig,
    http: Arc<dyn HttpClient>,
    cache: Arc<dyn CacheProvider>,
) -> Arc<Client> {
    return Arc::new(Client::new(http, environment, sdk_config, cache));
}
