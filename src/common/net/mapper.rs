use serde::de::DeserializeOwned;

use crate::common::dto::{BaseDto, BaseErrorDto};
use crate::common::result::{ErrorModel, NBResult};

pub(crate) fn map<D: DeserializeOwned>(response: &str) -> NBResult<D> {
    let dto: BaseDto<D> = serde_json::from_str(response)
        .map_err(|e| ErrorModel::network(format!("Failed to decode response: {e}")))?;

    if let Some(errors) = dto.errors.as_ref().filter(|e| !e.is_empty()) {
        return Err(map_errors(errors));
    }
    return dto.data.ok_or_else(|| ErrorModel::network("Please try again"));
}

fn map_errors(errors: &[BaseErrorDto]) -> ErrorModel {
    return match errors.first() {
        Some(error) => ErrorModel::network(error.message.clone())
            .with_code(error.extensions.classification.clone()),
        None => ErrorModel::network("Please try again"),
    };
}
