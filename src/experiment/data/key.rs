pub(super) const GATEWAY_OPERATION: &str = "experimentValue";

pub(super) const PARAM_KEY: &str = "key";
pub(super) const PARAM_PARAMETERS: &str = "parameters";

const CACHE_PREFIX: &str = "EXPERIMENT_";

pub(super) fn cache_key(key: &str) -> String {
    return format!("{CACHE_PREFIX}{key}");
}
