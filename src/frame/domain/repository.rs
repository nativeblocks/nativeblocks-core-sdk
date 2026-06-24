use std::collections::HashMap;

use crate::common::result::NBResult;
use crate::frame::domain::model::NativeFrameModel;

#[async_trait::async_trait]
pub(crate) trait FrameRepository: Send + Sync {
    async fn sync(&self, route: &str, parameters: &HashMap<String, String>) -> NBResult<NativeFrameModel>;

    fn get(&self, route: &str) -> NBResult<NativeFrameModel>;

    fn clear(&self, route: &str) -> NBResult<()>;

    fn clear_all(&self, routes: &[String]) -> NBResult<()>;
}
