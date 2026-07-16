mod client;
mod provider;

use crate::di::Container;
pub use client::GlobalParameterClient;
pub(crate) use provider::GlobalParameterProvider;
use std::sync::Arc;

pub(crate) fn build_provider() -> Arc<GlobalParameterProvider> {
    return Arc::new(GlobalParameterProvider::new());
}

pub(crate) fn get_or_create_client(container: &Container) -> Arc<GlobalParameterClient> {
    return container.component(|| {
        GlobalParameterClient::create(
            container.global_parameters(),
            container.logger(),
            container.sdk_config().clone(),
        )
    });
}
