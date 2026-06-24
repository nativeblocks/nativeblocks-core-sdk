use crate::common::result::{ErrorModel, NBResult};

use super::validation::is_valid_instance_name;
use super::{NativeblocksEdition, NativeblocksEnvironment};

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

    pub fn community_frame_endpoint(&self, route: &str) -> Option<String> {
        return match &self.edition {
            NativeblocksEdition::Community { frames_data } => frames_data.get(route).cloned(),
            NativeblocksEdition::Cloud { .. } => None,
        };
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
