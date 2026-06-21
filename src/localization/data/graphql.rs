use serde_json::{Value, json};

pub const LOCALIZATIONS_QUERY: &str = r#"query localizations($languageCode: String!) {
    localizations(languageCode: $languageCode) {
        checksum
        localizations {
            key
            value
        }
    }
}"#;

pub const LOCALIZATIONS_PRODUCTION_QUERY: &str =
    r#"query localizationsProduction($languageCode: String!) {
    localizationsProduction(languageCode: $languageCode) {
        checksum
        localizations {
            key
            value
        }
    }
}"#;

pub const PRODUCTION_CHECKSUM_QUERY: &str =
    r#"query localizationProductionChecksum($languageCode: String!) {
    localizationProductionChecksum(languageCode: $languageCode) {
        checksum
        languageCode
    }
}"#;

pub fn variables(language_code: &str) -> Value {
    json!({ "languageCode": language_code })
}
