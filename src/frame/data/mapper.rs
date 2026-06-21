use std::collections::HashMap;

use super::dto::{
    NativeActionDto, NativeActionTriggerDataDto, NativeActionTriggerDto,
    NativeActionTriggerPropertyDto, NativeBlockDataDto, NativeBlockDto, NativeBlockPropertyDto,
    NativeBlockSlotDto, NativeFrameDto, NativeVariableDto,
};
use crate::frame::domain::model::{
    NativeActionModel, NativeActionTriggerDataModel, NativeActionTriggerModel,
    NativeActionTriggerPropertyModel, NativeActionTriggerThen, NativeBlockDataModel,
    NativeBlockModel, NativeBlockPropertyModel, NativeBlockSlotModel, NativeFrameModel,
    NativeVariableModel,
};

fn or_empty(value: &Option<String>) -> String {
    value.clone().unwrap_or_default()
}

impl NativeFrameDto {
    pub(crate) fn to_model(&self) -> NativeFrameModel {
        NativeFrameModel {
            checksum: self.checksum.clone(),
            variables: Some(map_variables(&self.variables)),
            actions: Some(map_actions(&self.actions)),
            blocks: Some(map_blocks(&self.blocks)),
        }
    }
}

pub(crate) fn frame_to_model(dto: Option<&NativeFrameDto>) -> NativeFrameModel {
    match dto {
        Some(dto) => dto.to_model(),
        None => NativeFrameModel {
            checksum: None,
            variables: Some(HashMap::new()),
            actions: Some(HashMap::new()),
            blocks: Some(HashMap::new()),
        },
    }
}

fn map_variables(
    variables: &Option<Vec<Option<NativeVariableDto>>>,
) -> HashMap<String, NativeVariableModel> {
    let mut map = HashMap::new();
    if let Some(items) = variables {
        for item in items.iter().flatten() {
            let key = or_empty(&item.key);
            map.insert(
                key.clone(),
                NativeVariableModel {
                    key,
                    value: or_empty(&item.value),
                    value_type: or_empty(&item.value_type),
                },
            );
        }
    }
    map
}

fn map_blocks(blocks: &Option<Vec<Option<NativeBlockDto>>>) -> HashMap<String, NativeBlockModel> {
    let mut map = HashMap::new();
    if let Some(items) = blocks {
        for item in items.iter().flatten() {
            let key = or_empty(&item.key);
            map.insert(key.clone(), map_block(item, key.clone()));
        }
    }
    map
}

fn map_block(block: &NativeBlockDto, key: String) -> NativeBlockModel {
    NativeBlockModel {
        id: or_empty(&block.id),
        parent_id: or_empty(&block.parent_id),
        version: block.integration_version.unwrap_or(-1),
        slot: or_empty(&block.slot),
        key_type: or_empty(&block.key_type),
        key,
        visibility: or_empty(&block.visibility_key),
        position: block.position.unwrap_or(0),
        data: map_block_data(&block.data),
        properties: map_block_properties(&block.properties),
        slots: map_block_slots(&block.slots),
    }
}

fn map_block_data(
    data: &Option<Vec<Option<NativeBlockDataDto>>>,
) -> HashMap<String, NativeBlockDataModel> {
    let mut map = HashMap::new();
    if let Some(items) = data {
        for item in items.iter().flatten() {
            let key = or_empty(&item.key);
            map.insert(
                key.clone(),
                NativeBlockDataModel {
                    key,
                    value: or_empty(&item.value),
                    value_type: or_empty(&item.value_type),
                },
            );
        }
    }
    map
}

fn map_block_properties(
    properties: &Option<Vec<Option<NativeBlockPropertyDto>>>,
) -> HashMap<String, NativeBlockPropertyModel> {
    let mut map = HashMap::new();
    if let Some(items) = properties {
        for item in items.iter().flatten() {
            let key = or_empty(&item.key);
            map.insert(
                key.clone(),
                NativeBlockPropertyModel {
                    key,
                    value_mobile: or_empty(&item.value_mobile),
                    value_tablet: or_empty(&item.value_tablet),
                    value_desktop: or_empty(&item.value_desktop),
                    value_type: or_empty(&item.value_type),
                },
            );
        }
    }
    map
}

fn map_block_slots(
    slots: &Option<Vec<Option<NativeBlockSlotDto>>>,
) -> HashMap<String, NativeBlockSlotModel> {
    let mut map = HashMap::new();
    if let Some(items) = slots {
        for item in items.iter().flatten() {
            let slot = or_empty(&item.slot);
            map.insert(slot.clone(), NativeBlockSlotModel { slot });
        }
    }
    map
}

fn map_actions(
    actions: &Option<Vec<Option<NativeActionDto>>>,
) -> HashMap<String, Vec<NativeActionModel>> {
    let mut models: Vec<NativeActionModel> = Vec::new();
    let mut keys: Vec<String> = Vec::new();
    if let Some(items) = actions {
        for item in items.iter().flatten() {
            let key = or_empty(&item.key);
            models.push(map_action(item, key.clone()));
            keys.push(key);
        }
    }

    let mut map = HashMap::new();
    for key in keys {
        let grouped: Vec<NativeActionModel> =
            models.iter().filter(|a| a.key == key).cloned().collect();
        map.insert(key, grouped);
    }
    map
}

fn map_action(action: &NativeActionDto, key: String) -> NativeActionModel {
    let triggers = action
        .triggers
        .as_ref()
        .map(|items| items.iter().flatten().map(map_trigger).collect())
        .unwrap_or_default();
    NativeActionModel {
        id: or_empty(&action.id),
        key,
        event: or_empty(&action.event),
        triggers,
    }
}

fn map_trigger(trigger: &NativeActionTriggerDto) -> NativeActionTriggerModel {
    NativeActionTriggerModel {
        id: or_empty(&trigger.id),
        parent_id: or_empty(&trigger.parent_id),
        version: trigger.integration_version.unwrap_or(-1),
        name: or_empty(&trigger.name),
        key_type: or_empty(&trigger.key_type),
        then: NativeActionTriggerThen::from_then(&or_empty(&trigger.then)),
        properties: map_trigger_properties(&trigger.properties),
        data: map_trigger_data(&trigger.data),
    }
}

fn map_trigger_properties(
    properties: &Option<Vec<Option<NativeActionTriggerPropertyDto>>>,
) -> HashMap<String, NativeActionTriggerPropertyModel> {
    let mut map = HashMap::new();
    if let Some(items) = properties {
        for item in items.iter().flatten() {
            let key = or_empty(&item.key);
            map.insert(
                key.clone(),
                NativeActionTriggerPropertyModel {
                    key,
                    value: or_empty(&item.value),
                    value_type: or_empty(&item.value_type),
                },
            );
        }
    }
    map
}

fn map_trigger_data(
    data: &Option<Vec<Option<NativeActionTriggerDataDto>>>,
) -> HashMap<String, NativeActionTriggerDataModel> {
    let mut map = HashMap::new();
    if let Some(items) = data {
        for item in items.iter().flatten() {
            let key = or_empty(&item.key);
            map.insert(
                key.clone(),
                NativeActionTriggerDataModel {
                    key,
                    value: or_empty(&item.value),
                    value_type: or_empty(&item.value_type),
                },
            );
        }
    }
    map
}
