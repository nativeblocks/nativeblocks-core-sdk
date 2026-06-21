use std::collections::HashMap;

use serde::{Deserialize, Serialize};

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
pub enum NativeblocksEnvironment {
    Cloud {
        instance_name: String,
        endpoint: String,
        api_key: String,
        development_mode: bool,
    },
    Community {
        instance_name: String,
        frames_data: HashMap<String, String>,
    },
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
        match self {
            NativeblocksEnvironment::Cloud { instance_name, .. } => instance_name,
            NativeblocksEnvironment::Community { instance_name, .. } => instance_name,
        }
    }

    pub fn api_key(&self) -> &str {
        match self {
            NativeblocksEnvironment::Cloud { api_key, .. } => api_key,
            NativeblocksEnvironment::Community { .. } => "",
        }
    }

    pub fn endpoint(&self) -> Option<&str> {
        match self {
            NativeblocksEnvironment::Cloud { endpoint, .. } => Some(endpoint),
            NativeblocksEnvironment::Community { .. } => None,
        }
    }
}
