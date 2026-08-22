use crate::feature::frame::domain::model::NativeActionModel;
use crate::feature::frame::presenter::state_manager::state::InternalState;
use std::collections::HashMap;

pub(super) fn all(state: &InternalState) -> HashMap<String, Vec<NativeActionModel>> {
    return state.base.actions.clone();
}
