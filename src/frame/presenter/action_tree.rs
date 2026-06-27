use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::frame::domain::model::{
    NativeActionModel, NativeActionTriggerModel, NativeActionTriggerThen,
};
use crate::frame::presenter::action_props::{
    ActionProps, ChangeBlock, FindBlock, FindVariable, HandleTrigger, VariableChange,
};
use crate::frame::presenter::action_provider::ActionFinder;

#[allow(clippy::too_many_arguments)]
pub(super) async fn execute_action(
    instance_name: String,
    list_item_index: i32,
    action: NativeActionModel,
    performed_event_type: String,
    find_variable: FindVariable,
    variable_change: VariableChange,
    find_block: FindBlock,
    change_block: ChangeBlock,
    action_finder: Arc<dyn ActionFinder>,
) {
    if action.event != performed_event_type {
        return;
    }

    for trigger in root_triggers(&action) {
        execute_trigger(
            instance_name.clone(),
            list_item_index,
            action.clone(),
            trigger,
            find_variable.clone(),
            variable_change.clone(),
            find_block.clone(),
            change_block.clone(),
            action_finder.clone(),
        )
        .await;
    }
}

fn root_triggers(action: &NativeActionModel) -> Vec<NativeActionTriggerModel> {
    return action
        .triggers
        .iter()
        .filter(|trigger| trigger.parent_id.is_empty())
        .cloned()
        .collect();
}

fn sub_triggers(
    action: &NativeActionModel,
    parent_id: &str,
    then: NativeActionTriggerThen,
) -> Vec<NativeActionTriggerModel> {
    return action
        .triggers
        .iter()
        .filter(|trigger| trigger.parent_id == parent_id && trigger.then == then)
        .cloned()
        .collect();
}

#[allow(clippy::too_many_arguments)]
async fn execute_trigger(
    instance_name: String,
    list_item_index: i32,
    action: NativeActionModel,
    trigger: NativeActionTriggerModel,
    find_variable: FindVariable,
    variable_change: VariableChange,
    find_block: FindBlock,
    change_block: ChangeBlock,
    action_finder: Arc<dyn ActionFinder>,
) {
    let handler = match action_finder.find_handler(trigger.key_type.clone()) {
        Some(handler) => handler,
        None => {
            action_finder.fallback(trigger.key_type, trigger.name);
            return;
        }
    };

    let props = Arc::new(build_action_props(
        instance_name,
        list_item_index,
        action,
        trigger,
        find_variable,
        variable_change,
        find_block,
        change_block,
        action_finder,
    ));
    handler.handle(props).await;
}

#[allow(clippy::too_many_arguments)]
fn build_action_props(
    instance_name: String,
    list_item_index: i32,
    action: NativeActionModel,
    trigger: NativeActionTriggerModel,
    find_variable: FindVariable,
    variable_change: VariableChange,
    find_block: FindBlock,
    change_block: ChangeBlock,
    action_finder: Arc<dyn ActionFinder>,
) -> ActionProps {
    let make_trigger = |then: NativeActionTriggerThen| {
        trigger_handler(
            instance_name.clone(),
            list_item_index,
            action.clone(),
            find_variable.clone(),
            variable_change.clone(),
            find_block.clone(),
            change_block.clone(),
            action_finder.clone(),
            then,
        )
    };

    return ActionProps::new(
        instance_name.clone(),
        list_item_index,
        trigger,
        find_variable.clone(),
        variable_change.clone(),
        find_block.clone(),
        change_block.clone(),
        make_trigger(NativeActionTriggerThen::Next),
        make_trigger(NativeActionTriggerThen::Success),
        make_trigger(NativeActionTriggerThen::Failure),
    );
}

#[allow(clippy::too_many_arguments)]
fn trigger_handler(
    instance_name: String,
    list_item_index: i32,
    action: NativeActionModel,
    find_variable: FindVariable,
    variable_change: VariableChange,
    find_block: FindBlock,
    change_block: ChangeBlock,
    action_finder: Arc<dyn ActionFinder>,
    then: NativeActionTriggerThen,
) -> HandleTrigger {
    return Arc::new(move |trigger: NativeActionTriggerModel| {
        let instance_name = instance_name.clone();
        let action = action.clone();
        let find_variable = find_variable.clone();
        let variable_change = variable_change.clone();
        let find_block = find_block.clone();
        let change_block = change_block.clone();
        let action_finder = action_finder.clone();
        return Box::pin(async move {
            for child in sub_triggers(&action, &trigger.id, then) {
                execute_trigger(
                    instance_name.clone(),
                    list_item_index,
                    action.clone(),
                    child,
                    find_variable.clone(),
                    variable_change.clone(),
                    find_block.clone(),
                    change_block.clone(),
                    action_finder.clone(),
                )
                .await;
            }
        }) as Pin<Box<dyn Future<Output = ()> + Send>>;
    });
}
