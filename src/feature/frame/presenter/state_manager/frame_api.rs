use std::collections::HashMap;
use std::sync::Arc;

use crate::feature::frame::domain::model::NativeBlockModel;
use crate::feature::frame::presenter::state_manager::FrameStateManager;
use crate::feature::frame::presenter::state_manager::model::FrameChangeType;

#[uniffi::export(with_foreign)]
pub trait FrameStateObserver: Send + Sync {
    fn on_frame_change(&self, change: FrameChangeType);
}

#[uniffi::export(async_runtime = "tokio")]
impl FrameStateManager {
    pub async fn setup_frame(
        &self,
        route: String,
        args: HashMap<String, String>,
        observer: Arc<dyn FrameStateObserver>,
    ) {
        self.logger.set_route(&route);
        let globals = Arc::new(self.globals.get());
        self.observe(route.clone(), args, globals.clone(), observer);
        let _ = self.repository.load(&route, &globals).await;
    }

    pub fn release(&self) {
        self.release_subscription();
    }

    pub fn update_variable(&self, key: String, value: String) {
        self.change_variable(&key, value);
    }

    pub fn update_block(&self, block: NativeBlockModel) {
        self.change_block(&block);
    }
}
