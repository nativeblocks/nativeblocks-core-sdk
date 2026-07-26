pub(super) const LOCALIZATION_QUERY: &str = r#"query localizations($languageCode: String!) {
    localizations(languageCode: $languageCode) {
        checksum
        localizations {
            key
            value
        }
    }
}"#;

pub(super) const LOCALIZATION_PRODUCTION_QUERY: &str = r#"query localizationsProduction($languageCode: String!) {
    localizationsProduction(languageCode: $languageCode) {
        checksum
        localizations {
            key
            value
        }
    }
}"#;

pub(super) const LOCALIZATION_PRODUCTION_CHECKSUM_QUERY: &str = r#"query localizationProductionChecksum($languageCode: String!) {
    localizationProductionChecksum(languageCode: $languageCode) {
        checksum
        languageCode
    }
}"#;
