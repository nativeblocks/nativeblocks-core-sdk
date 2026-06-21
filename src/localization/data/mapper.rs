use std::collections::HashMap;

use super::dto::NativeLocalizationsDto;
use crate::localization::domain::model::NativeLocalizationModel;

impl NativeLocalizationsDto {
    pub(crate) fn to_model(&self) -> NativeLocalizationModel {
        let mut map = HashMap::new();
        if let Some(items) = &self.localizations {
            for item in items.iter().flatten() {
                map.insert(
                    item.key.clone().unwrap_or_default(),
                    item.value.clone().unwrap_or_default(),
                );
            }
        }
        NativeLocalizationModel {
            checksum: self.checksum.clone(),
            localizations: Some(map),
        }
    }
}

pub(crate) fn localization_to_model(
    dto: Option<&NativeLocalizationsDto>,
) -> NativeLocalizationModel {
    match dto {
        Some(dto) => dto.to_model(),
        None => NativeLocalizationModel {
            checksum: None,
            localizations: Some(HashMap::new()),
        },
    }
}
