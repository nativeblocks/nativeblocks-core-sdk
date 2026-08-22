use crate::feature::frame::domain::model::{NativeFrameModel, NativeVariableModel};
use crate::feature::frame::presenter::state_manager::model::RenderingState;
use crate::feature::frame::presenter::state_manager::variable;
use std::collections::HashMap;
use std::sync::Arc;

pub(super) struct InternalState {
    pub(super) base: Arc<NativeFrameModel>,
    pub(super) state: RenderingState,
    pub(super) root_key: Option<String>,
    pub(super) variables: HashMap<String, NativeVariableModel>,
}

impl InternalState {
    pub(super) fn fresh() -> Self {
        return Self::empty(RenderingState::Loading {});
    }

    pub(super) fn error(message: String) -> Self {
        return Self::empty(RenderingState::Error { message });
    }

    fn empty(state: RenderingState) -> Self {
        return Self {
            base: Arc::new(NativeFrameModel {
                checksum: None,
                variables: HashMap::new(),
                blocks: HashMap::new(),
                root_key: None,
                actions: HashMap::new(),
            }),
            state,
            root_key: None,
            variables: HashMap::new(),
        };
    }

    pub(super) fn ready(
        frame: Arc<NativeFrameModel>,
        args: &HashMap<String, String>,
        globals: &HashMap<String, String>,
    ) -> Self {
        let variables = variable::merge(frame.variables.clone(), args, globals);
        let root_key = frame.root_key.clone();
        return Self {
            base: frame,
            state: RenderingState::Ready {},
            root_key,
            variables,
        };
    }
}
