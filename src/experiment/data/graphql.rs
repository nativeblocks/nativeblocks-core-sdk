pub const GATEWAY_OPERATION: &str = "experimentValue";

pub const EXPERIMENT_QUERY: &str = r#"query experimentValue($key: String!, $parameter: ExperimentParameterInput) {
    experimentValue(key: $key, parameter: $parameter) {
        key
        value
        variableType
    }
}"#;
