pub(super) const GATEWAY_OPERATION: &str = "scaffold";

pub(super) fn cache_key(development_mode: bool) -> &'static str {
    return if development_mode {
        "SCAFFOLD_DEV"
    } else {
        "SCAFFOLD_PROD"
    };
}
