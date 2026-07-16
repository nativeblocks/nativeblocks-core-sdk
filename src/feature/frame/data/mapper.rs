use std::collections::HashMap;

use crate::feature::frame::data::dto::{
    NativeActionDto, NativeActionTriggerDataDto, NativeActionTriggerDto,
    NativeActionTriggerPropertyDto, NativeBlockDataDto, NativeBlockDto, NativeBlockPropertyDto,
    NativeBlockSlotDto, NativeFrameDto, NativeVariableDto,
};
use crate::feature::frame::domain::model::{
    NativeActionModel, NativeActionTriggerDataModel, NativeActionTriggerModel,
    NativeActionTriggerPropertyModel, NativeActionTriggerThen, NativeBlockDataModel,
    NativeBlockModel, NativeBlockPropertyModel, NativeBlockSlotModel, NativeFrameModel,
    NativeVariableModel,
};

const ROOT_KEY_TYPE: &str = "ROOT";

pub(super) fn to_model(dto: Option<NativeFrameDto>) -> NativeFrameModel {
    let Some(dto) = dto else {
        return NativeFrameModel {
            checksum: None,
            variables: HashMap::new(),
            blocks: HashMap::new(),
            root_id: None,
            actions: HashMap::new(),
        };
    };

    let NativeFrameDto {
        checksum,
        variables,
        blocks,
        actions,
        ..
    } = dto;

    let variables = keyed(variables, map_variable, |model| &model.key);
    let blocks = keyed(blocks, map_block, |model| &model.id);

    let root_id = blocks
        .values()
        .filter(|block| block.key_type == ROOT_KEY_TYPE)
        .min_by_key(|block| block.position)
        .map(|block| block.id.clone());

    let actions = grouped(actions, map_action, |model| &model.key);

    return NativeFrameModel {
        checksum,
        variables,
        blocks,
        root_id,
        actions,
    };
}

fn keyed<D, M>(
    items: Option<Vec<D>>,
    map: impl Fn(D) -> M,
    key: impl Fn(&M) -> &String,
) -> HashMap<String, M> {
    let items = items.unwrap_or_default();
    let mut out = HashMap::with_capacity(items.len());

    for item in items {
        let model = map(item);
        out.insert(key(&model).clone(), model);
    }
    return out;
}

fn grouped<D, M>(
    items: Option<Vec<D>>,
    map: impl Fn(D) -> M,
    key: impl Fn(&M) -> &String,
) -> HashMap<String, Vec<M>> {
    let items = items.unwrap_or_default();
    let mut out: HashMap<String, Vec<M>> = HashMap::with_capacity(items.len());

    for item in items {
        let model = map(item);
        match out.get_mut(key(&model)) {
            Some(bucket) => bucket.push(model),
            None => {
                out.insert(key(&model).clone(), vec![model]);
            }
        }
    }
    return out;
}

fn map_variable(dto: NativeVariableDto) -> NativeVariableModel {
    return NativeVariableModel {
        key: text(dto.key),
        value: text(dto.value),
        variable_type: text(dto.variable_type),
    };
}

fn map_block(dto: NativeBlockDto) -> NativeBlockModel {
    return NativeBlockModel {
        id: text(dto.id),
        parent_id: text(dto.parent_id),
        version: dto.integration_version.unwrap_or(-1),
        slot: text(dto.slot),
        key_type: text(dto.key_type),
        key: text(dto.key),
        visibility: text(dto.visibility_key),
        position: dto.position.unwrap_or(0),
        data: keyed(dto.data, map_block_data, |model| &model.key),
        properties: keyed(dto.properties, map_block_property, |model| &model.key),
        slots: keyed(dto.slots, map_block_slot, |model| &model.slot),
    };
}

fn map_block_data(dto: NativeBlockDataDto) -> NativeBlockDataModel {
    return NativeBlockDataModel {
        key: text(dto.key),
        value: text(dto.value),
        data_type: text(dto.data_type),
    };
}

fn map_block_property(dto: NativeBlockPropertyDto) -> NativeBlockPropertyModel {
    return NativeBlockPropertyModel {
        key: text(dto.key),
        value_mobile: text(dto.value_mobile),
        value_tablet: text(dto.value_tablet),
        value_desktop: text(dto.value_desktop),
        property_type: text(dto.property_type),
    };
}

fn map_block_slot(dto: NativeBlockSlotDto) -> NativeBlockSlotModel {
    return NativeBlockSlotModel {
        slot: text(dto.slot),
    };
}

fn map_action(dto: NativeActionDto) -> NativeActionModel {
    return NativeActionModel {
        id: text(dto.id),
        key: text(dto.key),
        event: text(dto.event),
        triggers: dto
            .triggers
            .unwrap_or_default()
            .into_iter()
            .map(map_trigger)
            .collect(),
    };
}

fn map_trigger(dto: NativeActionTriggerDto) -> NativeActionTriggerModel {
    return NativeActionTriggerModel {
        then: NativeActionTriggerThen::from_string(dto.then.as_deref().unwrap_or("")),
        name: text(dto.name),
        id: text(dto.id),
        parent_id: text(dto.parent_id),
        version: dto.integration_version.unwrap_or(-1),
        key_type: text(dto.key_type),
        properties: keyed(dto.properties, map_trigger_property, |model| &model.key),
        data: keyed(dto.data, map_trigger_data, |model| &model.key),
    };
}

fn map_trigger_property(dto: NativeActionTriggerPropertyDto) -> NativeActionTriggerPropertyModel {
    return NativeActionTriggerPropertyModel {
        key: text(dto.key),
        value: text(dto.value),
        property_type: text(dto.property_type),
    };
}

fn map_trigger_data(dto: NativeActionTriggerDataDto) -> NativeActionTriggerDataModel {
    return NativeActionTriggerDataModel {
        key: text(dto.key),
        value: text(dto.value),
        data_type: text(dto.data_type),
    };
}

fn text(value: Option<String>) -> String {
    return value.unwrap_or_default();
}
