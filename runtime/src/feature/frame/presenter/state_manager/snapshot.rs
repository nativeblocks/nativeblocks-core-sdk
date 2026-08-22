use crate::feature::frame::domain::model::NativeVariableModel;
use crate::feature::frame::presenter::state_manager::state::InternalState;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
#[derive(Debug, Clone)]
pub(crate) struct FrameSnapshot {
    variables: HashMap<String, NativeVariableModel>,
}

pub(super) struct SnapshotStore {
    key: Mutex<Option<String>>,
    snapshots: Arc<Mutex<HashMap<String, FrameSnapshot>>>,
}

impl SnapshotStore {
    pub(super) fn new(snapshots: Arc<Mutex<HashMap<String, FrameSnapshot>>>) -> Self {
        return Self {
            key: Mutex::new(None),
            snapshots,
        };
    }

    pub(super) fn open(&self, key: Option<String>) {
        let mut current = self.key.lock().unwrap();
        if *current == key {
            return;
        }
        if let Some(previous) = std::mem::replace(&mut *current, key) {
            self.snapshots.lock().unwrap().remove(&previous);
        }
    }

    pub(super) fn save(&self, state: &InternalState) {
        let Some(key) = self.key.lock().unwrap().clone() else {
            return;
        };
        self.snapshots.lock().unwrap().insert(key, capture(state));
    }

    pub(super) fn active(&self) -> Option<FrameSnapshot> {
        let key = self.key.lock().unwrap().clone()?;
        return self.snapshots.lock().unwrap().get(&key).cloned();
    }
}

fn capture(state: &InternalState) -> FrameSnapshot {
    return FrameSnapshot {
        variables: state.variables.clone(),
    };
}

pub(super) fn restore(state: &mut InternalState, snapshot: &FrameSnapshot) {
    for (key, variable) in &snapshot.variables {
        if state.variables.contains_key(key) {
            state.variables.insert(key.clone(), variable.clone());
        }
    }
}
