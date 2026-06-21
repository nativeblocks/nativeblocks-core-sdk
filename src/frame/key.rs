pub mod error_code {
    pub const FRAME_CACHE_EMPTY: &str = "NB0101";
    pub const FRAME_PRODUCTION_SYNC: &str = "NB0102";
    pub const FRAME_SYNC: &str = "NB0103";
    pub const FRAME_COMMUNITY_SYNC: &str = "NB0104";
    pub const FRAME_CHECKSUM: &str = "NB0105";
}

pub mod message {
    pub const CONNECTION_REQUIRED: &str = "Please make sure internet connection is available";
}

pub mod operation {
    pub const FRAME: &str = "frame";
    pub const FRAME_PRODUCTION: &str = "frameProduction";
    pub const FRAME_PRODUCTION_CHECKSUM: &str = "frameProductionChecksum";
}
