use std::collections::HashMap;

mod accessor;
mod validation;

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct SdkConfig {
    pub version: String,
    pub platform: String,
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum NativeblocksEdition {
    Cloud {
        endpoint: String,
        api_key: String,
        development_mode: bool,
    },
    Community {
        frames_data: HashMap<String, String>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct NativeblocksEnvironment {
    pub instance_name: String,
    pub edition: NativeblocksEdition,
}
