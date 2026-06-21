use std::collections::HashMap;

use serde_json::{Value, json};

pub const FRAME_QUERY: &str = r#"query frame($route: String!,$parameter: FrameParameterInput) {
    frame(route: $route, parameter: $parameter) {
        variables {
            key
            value
            type
        }
        blocks {
            id
            parentId
            integrationVersion
            slot
            keyType
            key
            visibilityKey
            position
            properties {
                key
                valueDesktop
                valueMobile
                valueTablet
                type
            }
            data {
                key
                value
                type
            }
            slots {
                slot
            }
        }
        actions {
            id
            key
            event
            triggers {
                id
                parentId
                integrationVersion
                name
                keyType
                then
                properties {
                    key
                    value
                    type
                }
                data {
                    key
                    value
                    type
                }
            }
        }
    }
}"#;

pub const FRAME_PRODUCTION_QUERY: &str = r#"query frameProduction($route: String!, $parameter: FrameParameterInput) {
    frameProduction(route: $route, parameter: $parameter) {
        checksum
        variables {
            key
            value
            type
        }
        blocks {
            id
            parentId
            slot
            keyType
            integrationVersion
            key
            visibilityKey
            position
            properties {
                key
                valueDesktop
                valueMobile
                valueTablet
                type
            }
            data {
                key
                value
                type
            }
            slots {
                slot
            }
        }
        actions {
            id
            key
            event
            triggers {
                id
                parentId
                name
                keyType
                then
                integrationVersion
                properties {
                    key
                    value
                    type
                }
                data {
                    key
                    value
                    type
                }
            }
        }
    }
}"#;

pub const FRAME_PRODUCTION_CHECKSUM_QUERY: &str =
    r#"query frameProductionChecksum($route: String!, $parameter: FrameParameterInput) {
    frameProductionChecksum(route: $route, parameter: $parameter) {
        checksum
        route
    }
}"#;

pub fn frame_variables(route: &str, parameters: &HashMap<String, String>) -> Value {
    let entries: Vec<Value> = parameters
        .iter()
        .map(|(key, value)| json!({ "key": key, "value": value }))
        .collect();
    json!({
        "route": route,
        "parameter": { "variables": entries },
    })
}
