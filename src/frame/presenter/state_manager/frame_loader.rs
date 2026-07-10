use std::collections::HashMap;
use std::sync::Arc;

use crate::common::result::{ErrorType, NBResult};
use crate::frame::domain::model::NativeFrameModel;

use super::block_tree::build_tree;
use super::{FrameState, FrameStateManager, FrameStateObserver};

impl FrameStateManager {
    pub(super) fn observe_cache(&self, route: String, args: HashMap<String, String>) {
        let mut frame_receiver = self.repository.get(&route);
        let weak_manager = self.me.clone();
        let task = tokio::spawn(async move {
            loop {
                let frame_result = frame_receiver.borrow_and_update().clone();
                let Some(manager) = weak_manager.upgrade() else { break };
                manager.apply_frame(frame_result, &args);
                if frame_receiver.changed().await.is_err() {
                    break;
                }
            }
        });
        if let Some(previous) = self.observe_task.lock().unwrap().replace(task) {
            previous.abort();
        }
    }

    fn apply_frame(&self, result: NBResult<NativeFrameModel>, args: &HashMap<String, String>) {
        let frame = match result {
            Ok(frame) => frame,
            Err(error) => {
                let state = if error.error_type == ErrorType::Cache {
                    FrameState::Loading {}
                } else {
                    FrameState::Error { message: error.message }
                };
                self.set_state(state);
                return;
            }
        };

        let dirty_block_ids: Vec<String> = {
            let mut state = self.state.lock().unwrap();

            let (variables, blocks, root_id, actions, variable_dependent_block_ids, frame_state) = build_tree(frame, args, &self.globals.get());
            state.variables = variables;
            state.blocks = blocks;
            state.root_id = root_id;
            state.actions = actions;
            state.variable_dependent_block_ids = variable_dependent_block_ids;
            state.frame_state = frame_state;

            state.block_observers.keys().cloned().collect()
        };

        self.logger.frame_state_changed(&FrameState::Ready {});
        if let Some(observer) = self.observer() {
            observer.on_state_changed(FrameState::Ready {});
        }

        self.notify_block_observers(&dirty_block_ids);
    }

    fn set_state(&self, state: FrameState) {
        self.state.lock().unwrap().frame_state = state.clone();
        self.logger.frame_state_changed(&state);
        if let Some(observer) = self.observer() {
            observer.on_state_changed(state);
        }
    }

    fn observer(&self) -> Option<Arc<dyn FrameStateObserver>> {
        return self.observer.lock().unwrap().clone();
    }
}
