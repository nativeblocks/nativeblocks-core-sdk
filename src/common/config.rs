use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::common::result::{ErrorModel, NBResult};

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct SdkConfig {
    pub version: String,
    pub platform: String,
}

impl SdkConfig {
    pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    pub const DEFAULT_PLATFORM: &'static str = "CORE";

    pub fn new(platform: impl Into<String>) -> Self {
        Self {
            version: Self::VERSION.to_string(),
            platform: platform.into(),
        }
    }
}

impl Default for SdkConfig {
    fn default() -> Self {
        Self::new(Self::DEFAULT_PLATFORM)
    }
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, uniffi::Record)]
pub struct ProjectConfigGateway {
    pub operation: String,
    #[serde(rename = "type")]
    pub gateway_type: String,
    pub value: String,
}

impl NativeblocksEnvironment {
    pub fn instance_name(&self) -> &str {
        return &self.instance_name;
    }

    pub fn api_key(&self) -> &str {
        return match &self.edition {
            NativeblocksEdition::Cloud { api_key, .. } => api_key,
            NativeblocksEdition::Community { .. } => "",
        };
    }

    pub fn endpoint(&self) -> Option<&str> {
        return match &self.edition {
            NativeblocksEdition::Cloud { endpoint, .. } => Some(endpoint),
            NativeblocksEdition::Community { .. } => None,
        };
    }

    pub fn development_mode(&self) -> bool {
        return matches!(
            self.edition,
            NativeblocksEdition::Cloud {
                development_mode: true,
                ..
            }
        );
    }

    pub fn is_community(&self) -> bool {
        return matches!(self.edition, NativeblocksEdition::Community { .. });
    }

    pub fn validate(&self) -> NBResult<()> {
        if is_valid_instance_name(&self.instance_name) {
            return Ok(());
        }
        return Err(ErrorModel::support(format!(
            "Please make sure the instance name '{}' contains valid characters: A-Z, a-z, 0-9, _ or -",
            self.instance_name
        )));
    }
}

fn is_valid_instance_name(name: &str) -> bool {
    return !name.trim().is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-');
}
