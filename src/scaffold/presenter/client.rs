use std::sync::Arc;

use crate::common::cache::CacheProvider;
use crate::common::environment::model::{NativeblocksEnvironment, SdkConfig};
use crate::common::net::HttpClient;
use crate::common::result::NBError;
use crate::di;
use crate::scaffold::domain::model::NativeScaffoldModel;

#[derive(uniffi::Object)]
pub struct ScaffoldClient {
    services: Arc<di::Services>,
}

#[uniffi::export(async_runtime = "tokio")]
impl ScaffoldClient {
    #[uniffi::constructor]
    pub fn new(
        environment: NativeblocksEnvironment,
        config: SdkConfig,
        http: Arc<dyn HttpClient>,
        cache: Arc<dyn CacheProvider>,
    ) -> Result<Arc<Self>, NBError> {
        let container = di::get_or_create(&environment, &config)?;
        let services = container.services(http, cache);
        return Ok(Arc::new(Self { services }));
    }

    pub async fn get_scaffold(&self) -> Result<NativeScaffoldModel, NBError> {
        return self
            .services
            .scaffold_repository()
            .fetch()
            .await
            .map_err(NBError::from);
    }
}
