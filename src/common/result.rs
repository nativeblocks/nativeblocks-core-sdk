use std::collections::HashMap;

use crate::common::logger::keys::parameter as param_key;

pub type NBResult<T> = Result<T, ErrorModel>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum ErrorType {
    Network,
    Cache,
    Support,
}

impl ErrorType {
    pub fn as_str(self) -> &'static str {
        match self {
            ErrorType::Network => "NETWORK",
            ErrorType::Cache => "CACHE",
            ErrorType::Support => "SUPPORT",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Error)]
pub enum NbError {
    Failure {
        reason: String,
        error_type: ErrorType,
        error_code: Option<String>,
    },
}

impl std::fmt::Display for NbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let NbError::Failure {
            reason,
            error_type,
            ..
        } = self;
        write!(f, "[{}] {}", error_type.as_str(), reason)
    }
}

impl std::error::Error for NbError {}

impl From<ErrorModel> for NbError {
    fn from(error: ErrorModel) -> Self {
        NbError::Failure {
            reason: error.message,
            error_type: error.error_type,
            error_code: error.error_code,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorModel {
    pub message: String,
    pub error_type: ErrorType,
    pub error_code: Option<String>,
}

impl ErrorModel {
    pub fn new(message: impl Into<String>, error_type: ErrorType) -> Self {
        Self {
            message: message.into(),
            error_type,
            error_code: None,
        }
    }

    pub fn network(message: impl Into<String>) -> Self {
        Self::new(message, ErrorType::Network)
    }

    pub fn cache(message: impl Into<String>) -> Self {
        Self::new(message, ErrorType::Cache)
    }

    pub fn support(message: impl Into<String>) -> Self {
        Self::new(message, ErrorType::Support)
    }

    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.error_code = Some(code.into());
        self
    }

    pub fn to_logger_parameters(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert(param_key::ERROR_MESSAGE.to_string(), self.message.clone());
        map.insert(
            param_key::ERROR_TYPE.to_string(),
            self.error_type.as_str().to_string(),
        );
        if let Some(code) = &self.error_code {
            map.insert(param_key::ERROR_TAG.to_string(), code.clone());
        }
        map
    }
}

impl std::fmt::Display for ErrorModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.error_type.as_str(), self.message)
    }
}

impl std::error::Error for ErrorModel {}
