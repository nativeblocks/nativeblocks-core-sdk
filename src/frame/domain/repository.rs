use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::watch;

use crate::common::result::NBResult;
use crate::frame::domain::model::NativeFrameModel;

pub(crate) type FrameUpdate = NBResult<Arc<NativeFrameModel>>;

#[async_trait::async_trait]
pub(crate) trait FrameRepository: Send + Sync {
    async fn load(&self, route: &str, parameters: &HashMap<String, String>) -> NBResult<()>;

    async fn sync(&self, route: &str, parameters: &HashMap<String, String>) -> NBResult<()>;

    fn subscribe(&self, route: &str) -> watch::Receiver<FrameUpdate>;

    async fn clear(&self, route: &str) -> NBResult<()>;

    async fn clear_all(&self, routes: &[String]) -> NBResult<()>;
}
