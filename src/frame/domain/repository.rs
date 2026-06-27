use std::collections::HashMap;

use tokio::sync::watch;

use crate::common::result::NBResult;
use crate::frame::domain::model::NativeFrameModel;

#[async_trait::async_trait]
pub(crate) trait FrameRepository: Send + Sync {
    async fn sync(&self, route: &str, parameters: &HashMap<String, String>) -> NBResult<()>;

    fn get(&self, route: &str) -> watch::Receiver<NBResult<NativeFrameModel>>;

    async fn clear(&self, route: &str) -> NBResult<()>;

    async fn clear_all(&self, routes: &[String]) -> NBResult<()>;
}
