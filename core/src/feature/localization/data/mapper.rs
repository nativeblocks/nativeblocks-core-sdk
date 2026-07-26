use std::collections::HashMap;

use crate::feature::localization::data::dto::NativeLocalizationsDto;
use crate::feature::localization::domain::model::NativeLocalizationModel;

pub(super) fn to_model(dto: Option<&NativeLocalizationsDto>) -> NativeLocalizationModel {
    let dto = match dto {
        Some(dto) => dto,
        None => {
            return NativeLocalizationModel {
                checksum: None,
                localizations: HashMap::new(),
            };
        }
    };

    let localizations = dto
        .localizations
        .iter()
        .flatten()
        .map(|item| (text(&item.key), text(&item.value)))
        .collect();

    return NativeLocalizationModel {
        checksum: dto.checksum.clone(),
        localizations,
    };
}

fn text(value: &Option<String>) -> String {
    return value.clone().unwrap_or_default();
}
