//! Tests for `common::net`.

use super::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
struct Payload {
    name: String,
}

#[test]
fn decodes_success() {
    let body = r#"{"data":{"name":"frame"}}"#;
    assert_eq!(
        decode_envelope::<Payload>(body).unwrap(),
        Payload {
            name: "frame".into()
        }
    );
}

#[test]
fn errors_take_precedence_with_classification() {
    let body = r#"{"errors":[{"message":"boom","extensions":{"classification":"BAD"}}]}"#;
    let err = decode_envelope::<Payload>(body).unwrap_err();
    assert_eq!(err.message, "boom");
    assert_eq!(err.error_code.as_deref(), Some("BAD"));
}

#[test]
fn missing_data_is_generic_error() {
    let err = decode_envelope::<Payload>("{}").unwrap_err();
    assert_eq!(err.message, "Please try again");
}

#[test]
fn derives_operation_name_from_query() {
    assert_eq!(operation_name_from_query("query scaffold { scaffold { id } }"), "scaffold");
    assert_eq!(operation_name_from_query("mutation UpdateUser($id: ID!) { ok }"), "UpdateUser");
    assert_eq!(operation_name_from_query("query frame{frame{id}}"), "frame");
    assert_eq!(operation_name_from_query("{ anonymous }"), "");
}

#[test]
fn request_serializes_operation_name_camel_case() {
    let request = GraphQlRequest::new("query scaffold { scaffold { id } }");
    assert_eq!(request.operation_name, "scaffold");
    let json = serde_json::to_value(&request).unwrap();
    assert_eq!(json["operationName"], "scaffold");
}

#[test]
fn auth_headers_carry_bearer_and_sdk_identity() {
    let env = NativeblocksEnvironment::Cloud {
        instance_name: "i".into(),
        endpoint: "https://x".into(),
        api_key: "k".into(),
        development_mode: false,
    };
    let cfg = SdkConfig::new("TEST");
    let headers = with_headers(&env, &cfg);
    assert!(headers.contains(&("Api-Key".to_string(), "Bearer k".to_string())));
    assert!(headers.contains(&("SDK-Platform".to_string(), "TEST".to_string())));
}
