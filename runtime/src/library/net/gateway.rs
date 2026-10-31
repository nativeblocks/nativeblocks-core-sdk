use crate::library::net::binary::GATEWAY_TYPE_BINARY;
use crate::library::net::rest::GATEWAY_TYPE_REST;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GatewayType {
    GraphQl,
    Rest,
    Binary,
}

impl GatewayType {
    pub(crate) fn of(value: &str) -> Self {
        if value.eq_ignore_ascii_case(GATEWAY_TYPE_BINARY) {
            return GatewayType::Binary;
        }
        if value.eq_ignore_ascii_case(GATEWAY_TYPE_REST) {
            return GatewayType::Rest;
        }
        return GatewayType::GraphQl;
    }
}
