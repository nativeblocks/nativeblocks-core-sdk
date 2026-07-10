use std::collections::HashMap;

use crate::frame::data::dto::{
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

const ROOT_KEY_TYPE: &str = "ROOT";

pub(super) fn to_model(dto: Option<&NativeFrameDto>) -> NativeFrameModel {
    let dto = match dto {
        Some(dto) => dto,
        None => {
            return NativeFrameModel {
                checksum: None,
                variables: HashMap::new(),
                blocks: HashMap::new(),
                root_id: None,
                actions: HashMap::new(),
            };
        }
    };

    let variables = dto
        .variables
        .iter()
        .flatten()
        .map(|variable| (text(&variable.key), map_variable(variable)))
        .collect();

    let blocks = group_blocks(dto);
    let root_id = blocks
        .values()
        .flatten()
        .filter(|block| block.key_type == ROOT_KEY_TYPE)
        .min_by_key(|block| block.position)
        .map(|block| block.id.clone());

    let mut actions: HashMap<String, Vec<NativeActionModel>> = HashMap::new();
    for action in dto.actions.iter().flatten() {
        let model = map_action(action);
        actions.entry(model.key.clone()).or_default().push(model);
    }

    return NativeFrameModel {
        checksum: dto.checksum.clone(),
        variables,
        blocks,
        root_id,
        actions,
    };
}

fn group_blocks(dto: &NativeFrameDto) -> HashMap<String, Vec<NativeBlockModel>> {
    let mut map: HashMap<String, Vec<NativeBlockModel>> = HashMap::new();
    for block in dto.blocks.iter().flatten() {
        let model = map_block(block);
        map.entry(model.parent_id.clone()).or_default().push(model);
    }
    for children in map.values_mut() {
        children.sort_by_key(|block| block.position);
    }
    return map;
}

fn map_variable(dto: &NativeVariableDto) -> NativeVariableModel {
    return NativeVariableModel {
        key: text(&dto.key),
        value: text(&dto.value),
        variable_type: text(&dto.variable_type),
    };
}

fn map_block(dto: &NativeBlockDto) -> NativeBlockModel {
    return NativeBlockModel {
        id: text(&dto.id),
        parent_id: text(&dto.parent_id),
        version: dto.integration_version.unwrap_or(-1),
        slot: text(&dto.slot),
        key_type: text(&dto.key_type),
        key: text(&dto.key),
        visibility: text(&dto.visibility_key),
        position: dto.position.unwrap_or(0),
        data: dto
            .data
            .iter()
            .flatten()
            .map(|data| (text(&data.key), map_block_data(data)))
            .collect(),
        properties: dto
            .properties
            .iter()
            .flatten()
            .map(|property| (text(&property.key), map_block_property(property)))
            .collect(),
        slots: dto
            .slots
            .iter()
            .flatten()
            .map(|slot| (text(&slot.slot), map_block_slot(slot)))
            .collect(),
    };
}

fn map_block_data(dto: &NativeBlockDataDto) -> NativeBlockDataModel {
    return NativeBlockDataModel {
        key: text(&dto.key),
        value: text(&dto.value),
        data_type: text(&dto.data_type),
    };
}

fn map_block_property(dto: &NativeBlockPropertyDto) -> NativeBlockPropertyModel {
    return NativeBlockPropertyModel {
        key: text(&dto.key),
        value_mobile: text(&dto.value_mobile),
        value_tablet: text(&dto.value_tablet),
        value_desktop: text(&dto.value_desktop),
        property_type: text(&dto.property_type),
    };
}

fn map_block_slot(dto: &NativeBlockSlotDto) -> NativeBlockSlotModel {
    return NativeBlockSlotModel {
        slot: text(&dto.slot),
    };
}

fn map_action(dto: &NativeActionDto) -> NativeActionModel {
    return NativeActionModel {
        id: text(&dto.id),
        key: text(&dto.key),
        event: text(&dto.event),
        triggers: group_triggers(dto),
    };
}

fn group_triggers(dto: &NativeActionDto) -> HashMap<String, Vec<NativeActionTriggerModel>> {
    let mut map: HashMap<String, Vec<NativeActionTriggerModel>> = HashMap::new();
    for trigger in dto.triggers.iter().flatten() {
        let model = map_trigger(trigger);
        map.entry(model.parent_id.clone()).or_default().push(model);
    }
    return map;
}

fn map_trigger(dto: &NativeActionTriggerDto) -> NativeActionTriggerModel {
    return NativeActionTriggerModel {
        name: text(&dto.name),
        id: text(&dto.id),
        parent_id: text(&dto.parent_id),
        version: dto.integration_version.unwrap_or(-1),
        key_type: text(&dto.key_type),
        then: NativeActionTriggerThen::from_string(dto.then.as_deref().unwrap_or("")),
        properties: dto
            .properties
            .iter()
            .flatten()
            .map(|property| (text(&property.key), map_trigger_property(property)))
            .collect(),
        data: dto
            .data
            .iter()
            .flatten()
            .map(|data| (text(&data.key), map_trigger_data(data)))
            .collect(),
        sub_triggers: None,
    };
}

fn map_trigger_property(dto: &NativeActionTriggerPropertyDto) -> NativeActionTriggerPropertyModel {
    return NativeActionTriggerPropertyModel {
        key: text(&dto.key),
        value: text(&dto.value),
        property_type: text(&dto.property_type),
    };
}

fn map_trigger_data(dto: &NativeActionTriggerDataDto) -> NativeActionTriggerDataModel {
    return NativeActionTriggerDataModel {
        key: text(&dto.key),
        value: text(&dto.value),
        data_type: text(&dto.data_type),
    };
}

fn text(value: &Option<String>) -> String {
    return value.clone().unwrap_or_default();
}
