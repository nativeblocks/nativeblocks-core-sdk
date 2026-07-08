use std::sync::Arc;

use crate::common::cache::CacheProvider;
use crate::common::environment::{NativeblocksEnvironment, SdkConfig};
use crate::common::net::HttpClient;
use crate::common::result::NBError;
use crate::scaffold::di::container::Container;
use crate::scaffold::domain::model::NativeScaffoldModel;

#[derive(uniffi::Object)]
pub struct ScaffoldClient {
    container: Arc<Container>,
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
        environment.validate()?;
        let container = Arc::new(Container::new(environment, config, http, cache)?);
        return Ok(Arc::new(Self { container }));
    }

    pub async fn get_scaffold(&self) -> Result<NativeScaffoldModel, NBError> {
        return self
            .container
            .repository()
            .fetch()
            .await
            .map_err(NBError::from);
    }
}
