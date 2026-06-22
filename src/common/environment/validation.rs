pub(super) fn is_valid_instance_name(name: &str) -> bool {
    return !name.trim().is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-');
}
