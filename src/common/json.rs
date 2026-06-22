use serde::Serialize;

use crate::common::result::{ErrorModel, NBResult};

pub(crate) fn to_json<T: Serialize>(value: &T) -> NBResult<String> {
    return serde_json::to_string(value).map_err(|e| ErrorModel::cache(e.to_string()));
}
