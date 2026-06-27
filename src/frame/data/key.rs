pub(super) const GATEWAY_FRAME: &str = "frame";
pub(super) const GATEWAY_FRAME_PRODUCTION: &str = "frameProduction";
pub(super) const GATEWAY_FRAME_PRODUCTION_CHECKSUM: &str = "frameProductionChecksum";

pub(super) const PARAM_ROUTE: &str = "route";
pub(super) const PARAM_PARAMETERS: &str = "parameters";

pub(super) fn dev_key(route: &str) -> String {
    return format!("FRAME_DEV::{route}");
}

pub(super) fn prod_key(route: &str) -> String {
    return format!("FRAME_PROD::{route}");
}

pub(in crate::frame) mod error_code {
    pub(in crate::frame) const FRAME_NOT_CACHED: &str = "NB0101";
    pub(in crate::frame) const FRAME_PRODUCTION_SYNC: &str = "NB0102";
    pub(in crate::frame) const FRAME_DEV_SYNC: &str = "NB0103";
    pub(in crate::frame) const FRAME_CHECKSUM: &str = "NB0105";
}

pub(in crate::frame) mod message {
    pub(in crate::frame) const FRAME_NOT_CACHED: &str = "Please make sure internet connection is available";
}
