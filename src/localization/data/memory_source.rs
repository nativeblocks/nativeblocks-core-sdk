use std::collections::HashMap;
use std::sync::Mutex;

use crate::localization::domain::model::NativeLocalizationModel;

pub(super) struct MemoryLocalizationSource {
    localizations: Mutex<HashMap<String, NativeLocalizationModel>>,
}

impl MemoryLocalizationSource {
    pub(super) fn new() -> Self {
        return Self {
            localizations: Mutex::new(HashMap::new()),
        };
    }

    pub(super) fn get_localization(&self, language_code: &str) -> Option<NativeLocalizationModel> {
        return self.localizations.lock().unwrap().get(language_code).cloned();
    }

    pub(super) fn save_localization(&self, language_code: &str, localization: NativeLocalizationModel) {
        self.localizations.lock().unwrap().insert(language_code.to_string(), localization);
    }

    pub(super) fn translate(&self, language_code: &str, key: &str) -> Option<String> {
        let localizations = self.localizations.lock().unwrap();
        let localization = localizations.get(language_code);
        if localization.is_none() {
            return None;
        }
        return localization.unwrap().localizations.get(key).cloned();
    }
}
