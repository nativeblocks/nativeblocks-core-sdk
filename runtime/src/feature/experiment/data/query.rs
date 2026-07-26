pub(super) const EXPERIMENT_QUERY: &str = r#"query experimentValue($key: String!, $parameter: ExperimentParameterInput) {
    experimentValue(key: $key, parameter: $parameter) {
        key
        value
        variableType
    }
}"#;
