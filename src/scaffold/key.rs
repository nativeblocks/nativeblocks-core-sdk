//! Scaffold feature constants. Ports the inline constants in
//! `ScaffoldRepositoryImpl.kt` plus `GetScaffold.graphql`.

/// Header carrying the install id on scaffold requests.
pub const INSTALL_ID_HEADER: &str = "Install-Id";

/// The config-gateway operation name for the scaffold.
pub const GATEWAY_OPERATION: &str = "scaffold";

/// Gateway type that selects the REST (HTTP GET) path; anything else is GraphQL.
pub const GATEWAY_TYPE_REST: &str = "rest";

/// The GraphQL document used by the non-REST path. Ports `GetScaffold.graphql`.
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
