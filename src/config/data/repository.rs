use std::sync::Arc;

use tokio::sync::Mutex as AsyncMutex;
use uuid::Uuid;

use crate::common::cache::CacheProvider;
use crate::common::result::NBResult;
use crate::config::data::source::ProjectConfigRemoteSource;
use crate::config::key::INSTALL_ID_KEY;
use crate::config::model::NativeProjectConfigModel;

pub(crate) struct ProjectConfigRepository {
    source: ProjectConfigRemoteSource,
    local_cache: Arc<dyn CacheProvider>,
    config_cache: AsyncMutex<Option<NativeProjectConfigModel>>,
}

impl ProjectConfigRepository {
    pub(crate) fn new(
        source: ProjectConfigRemoteSource,
        local_cache: Arc<dyn CacheProvider>,
    ) -> Self {
        return Self {
            source,
            local_cache,
            config_cache: AsyncMutex::new(None),
        };
    }

    pub(crate) fn install_id(&self) -> NBResult<String> {
        let existing = self.local_cache.get_string(INSTALL_ID_KEY, "")?;
        if !existing.is_empty() {
            return Ok(existing);
        }
        let install_id = Uuid::now_v7().to_string();
        self.local_cache
            .save_string(INSTALL_ID_KEY, &install_id, None)?;
        return Ok(install_id);
    }

    pub(crate) async fn project_config(
        &self,
        install_id: &str,
    ) -> NBResult<NativeProjectConfigModel> {
        let mut cache = self.config_cache.lock().await;
        if let Some(model) = cache.as_ref() {
            return Ok(model.clone());
        }
        let dto = self.source.fetch(install_id).await?;
        let model = dto.to_model();
        *cache = Some(model.clone());
        return Ok(model);
    }
}
