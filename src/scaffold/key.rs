pub const GATEWAY_OPERATION: &str = "scaffold";

pub const SCAFFOLD_QUERY: &str = r#"query scaffold {
    scaffold {
        frames {
            id
            name
            route
            type
            platform
            routeArguments {
                name
            }
        }
    }
}"#;
