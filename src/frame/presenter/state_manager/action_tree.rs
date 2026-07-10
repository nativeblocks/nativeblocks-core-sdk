use std::collections::HashMap;
use std::sync::Arc;

use crate::frame::domain::model::{NativeActionModel, NativeActionTriggerModel, NativeActionTriggerThen};
use crate::frame::presenter::action_context::ActionContext;
use crate::frame::presenter::script_runner;

use super::FrameStateManager;

const SCRIPT_KEY_TYPE: &str = "SCRIPT";

impl FrameStateManager {

    pub(crate) fn action_of(&self, block_id: &str, event: &str) -> Option<NativeActionModel> {
        let state = self.state.lock().unwrap();
        let block_key = &state
            .blocks
            .values()
            .flatten()
            .find(|block| block.id == block_id)?
            .key;
        return state
            .actions
            .get(block_key)?
            .iter()
            .find(|action| action.event == event)
            .cloned();
    }

    pub(crate) fn handle_event(self: &Arc<Self>, index: i32, block_id: &str, event: &str) {
        match self.action_of(block_id, event) {
            Some(action) => self.execute_action(index, &action),
            None => self.logger.action_ignored(block_id, event),
        }
    }

    pub(crate) fn advance(
        self: &Arc<Self>,
        index: i32,
        parent: &NativeActionTriggerModel,
        then: NativeActionTriggerThen,
        triggers: &Arc<HashMap<String, Vec<NativeActionTriggerModel>>>,
        action_label: &Arc<str>,
    ) {
        let children = triggers.get(&parent.id).cloned().unwrap_or_default();
        for child in children.into_iter().filter(|child| child.then == then) {
            self.execute_trigger(index, child, triggers.clone(), action_label.clone());
        }
    }

    fn execute_action(self: &Arc<Self>, index: i32, action: &NativeActionModel) {
        self.logger.action_triggered(&action.key, &action.event);
        let action_label: Arc<str> = Arc::from(format!("{}[{}]", action.key, action.event));
        let triggers = Arc::new(action.triggers.clone());
        let root_triggers = triggers.get("").cloned().unwrap_or_default();
        for trigger in root_triggers {
            self.execute_trigger(index, trigger, triggers.clone(), action_label.clone());
        }
    }

    fn execute_trigger(
        self: &Arc<Self>,
        index: i32,
        trigger: NativeActionTriggerModel,
        triggers: Arc<HashMap<String, Vec<NativeActionTriggerModel>>>,
        action_label: Arc<str>,
    ) {
        self.logger.trigger_executed(&action_label, &trigger);
        if trigger.key_type == SCRIPT_KEY_TYPE {
            script_runner::run_script(self, index, &trigger);
            self.advance(index, &trigger, NativeActionTriggerThen::Next, &triggers, &action_label);
            return;
        }
        let key_type = trigger.key_type.clone();
        let trigger_name = trigger.name.clone();
        let props = ActionContext::new(self.clone(), index, trigger, triggers, action_label);
        match self.action_dispatcher.lock().unwrap().clone() {
            Some(dispatcher) => dispatcher.dispatch(key_type, props),
            None => self.logger.action_fallback(&key_type, &trigger_name),
        }
    }
}
