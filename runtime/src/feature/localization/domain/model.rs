use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, uniffi::Record)]
pub struct NativeLocalizationModel {
    pub checksum: Option<String>,
    pub localizations: HashMap<String, String>,
}
