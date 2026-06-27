use crate::common::result::{ErrorModel, NBResult};

use super::NativeblocksEnvironment;
use super::validation::is_valid_instance_name;

impl NativeblocksEnvironment {
    pub fn instance_name(&self) -> &str {
        return &self.instance_name;
    }

    pub fn api_key(&self) -> &str {
        return &self.api_key;
    }

    pub fn endpoint(&self) -> &str {
        return &self.endpoint;
    }

    pub fn development_mode(&self) -> bool {
        return self.development_mode;
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
