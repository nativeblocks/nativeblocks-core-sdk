use crate::feature::frame::presenter::state_manager::frame_api::FrameStateObserver;
use crate::feature::frame::presenter::state_manager::model::FrameChangeType;
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;

pub(super) struct Observer {
    active: Mutex<Option<Active>>,
}

struct Active {
    observer: Arc<dyn FrameStateObserver>,
    task: JoinHandle<()>,
}

impl Drop for Active {
    fn drop(&mut self) {
        self.task.abort();
    }
}

impl Observer {
    pub(super) fn empty() -> Self {
        return Self {
            active: Mutex::new(None),
        };
    }

    pub(super) fn set(&self, observer: Arc<dyn FrameStateObserver>, task: JoinHandle<()>) {
        *self.active.lock().unwrap() = Some(Active { observer, task });
    }

    pub(super) fn clear(&self) {
        self.active.lock().unwrap().take();
    }

    pub(super) fn emit(&self, change: FrameChangeType) {
        let Some(observer) = self.observer() else {
            return;
        };
        observer.on_frame_change(change);
    }

    fn observer(&self) -> Option<Arc<dyn FrameStateObserver>> {
        return self
            .active
            .lock()
            .unwrap()
            .as_ref()
            .map(|active| active.observer.clone());
    }
}
