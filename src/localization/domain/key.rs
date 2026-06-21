pub mod error_code {
    pub const CACHE_EMPTY: &str = "NB0201";
    pub const PRODUCTION_SYNC: &str = "NB0202";
    pub const SYNC: &str = "NB0203";
    pub const CHECKSUM: &str = "NB0204";
}

pub mod message {
    pub const CONNECTION_REQUIRED: &str = "Please make sure internet connection is available";
    pub const CLOUD_ONLY: &str =
        "To sync localization data from remote, you need to use cloud edition";
}

pub mod operation {
    pub const LOCALIZATIONS: &str = "localizations";
    pub const LOCALIZATIONS_PRODUCTION: &str = "localizationsProduction";
    pub const PRODUCTION_CHECKSUM: &str = "localizationProductionChecksum";
}
