use crate::feature::frame::domain::model::{NativeActionModel, NativeActionTriggerModel};
use crate::feature::frame::presenter::logging::FrameLogger;
use crate::feature::frame::presenter::state_manager::model::ActionLogEvent;
use crate::feature::frame::presenter::state_manager::state::InternalState;
use std::collections::HashMap;

pub(super) fn render(
    state: &InternalState,
    logger: &FrameLogger,
) -> HashMap<String, Vec<NativeActionModel>> {
    return state
        .base
        .actions
        .iter()
        .map(|(key, actions)| {
            let placed = actions
                .iter()
                .map(|action| with_scoped_triggers(action, logger));
            return (key.clone(), placed.collect());
        })
        .collect();
}

fn with_scoped_triggers(action: &NativeActionModel, logger: &FrameLogger) -> NativeActionModel {
    let by_id: HashMap<&str, &NativeActionTriggerModel> = action
        .triggers
        .iter()
        .map(|trigger| (trigger.id.as_str(), trigger))
        .collect();

    let mut placed = action.clone();
    placed.triggers = action
        .triggers
        .iter()
        .filter(|trigger| fits(trigger, &by_id, logger))
        .cloned()
        .collect();
    return placed;
}

fn fits(
    trigger: &NativeActionTriggerModel,
    by_id: &HashMap<&str, &NativeActionTriggerModel>,
    logger: &FrameLogger,
) -> bool {
    let required = trigger.scope.clone().unwrap_or_default();
    let provided = provided_scope(trigger, by_id);
    if required.is_empty() || required == provided {
        return true;
    }
    let dropped = !provided.is_empty();
    logger.action(ActionLogEvent::ScopeMismatch {
        trigger_name: trigger.name.clone(),
        key_type: trigger.key_type.clone(),
        required,
        provided,
        dropped,
    });
    return !dropped;
}

fn provided_scope(
    trigger: &NativeActionTriggerModel,
    by_id: &HashMap<&str, &NativeActionTriggerModel>,
) -> String {
    let Some(parent) = by_id.get(trigger.parent_id.as_str()) else {
        return String::new();
    };
    let Some(event) = parent.events.get(&trigger.event) else {
        return String::new();
    };
    return event.scope.clone().unwrap_or_default();
}
