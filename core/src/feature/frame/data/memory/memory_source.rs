use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::feature::frame::domain::model::NativeFrameModel;

pub(crate) struct MemoryFrameSource {
    frames: Mutex<HashMap<String, Arc<NativeFrameModel>>>,
}

impl MemoryFrameSource {
    pub(crate) fn new() -> Self {
        return Self {
            frames: Mutex::new(HashMap::new()),
        };
    }

    pub(crate) fn get_frame(&self, route: &str) -> Option<Arc<NativeFrameModel>> {
        return self.frames.lock().unwrap().get(route).cloned();
    }

    pub(crate) fn save_frame(&self, route: &str, frame: Arc<NativeFrameModel>) {
        self.frames.lock().unwrap().insert(route.to_string(), frame);
    }

    pub(crate) fn clear(&self, route: &str) {
        self.frames.lock().unwrap().remove(route);
    }

    pub(crate) fn clear_all(&self, routes: &[String]) {
        let mut frames = self.frames.lock().unwrap();
        for route in routes {
            frames.remove(route);
        }
    }
}
