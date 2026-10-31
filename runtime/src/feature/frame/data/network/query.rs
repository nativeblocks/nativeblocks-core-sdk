pub(in crate::feature::frame::data) const FRAME_QUERY: &str = r#"query frame($route: String!,$parameter: FrameParameterInput) {
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
            scope
            visibilityKey
            position
            data {
                key
                value
                type
            }
            slots {
                slot
                scope
            }
            modifiers {
                keyType
                position
                scope
                data {
                    key
                    value
                    type
                }
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
                scope
                data {
                    key
                    value
                    type
                }
                events {
                    event
                    scope
                }
            }
        }
    }
}"#;

pub(in crate::feature::frame::data) const FRAME_PRODUCTION_QUERY: &str = r#"query frameProduction($route: String!, $parameter: FrameParameterInput) {
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
            scope
            visibilityKey
            position
            data {
                key
                value
                type
            }
            slots {
                slot
                scope
            }
            modifiers {
                keyType
                position
                scope
                data {
                    key
                    value
                    type
                }
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
                scope
                integrationVersion
                data {
                    key
                    value
                    type
                }
                events {
                    event
                    scope
                }
            }
        }
    }
}"#;

pub(in crate::feature::frame::data) const FRAME_PRODUCTION_CHECKSUM_QUERY: &str = r#"query frameProductionChecksum($route: String!, $parameter: FrameParameterInput) {
    frameProductionChecksum(route: $route, parameter: $parameter) {
        checksum
        route
    }
}"#;
