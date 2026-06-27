pub(super) const SCAFFOLD_QUERY: &str = r#"query scaffold {
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
