mod accessor;
mod validation;

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct SdkConfig {
    pub version: String,
    pub platform: String,
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct NativeblocksEnvironment {
    pub instance_name: String,
    pub endpoint: String,
    pub api_key: String,
    pub development_mode: bool,
}
