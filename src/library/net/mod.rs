mod dto;
mod graphql;
mod mapper;
pub mod network;
mod rest;

pub(crate) use graphql::{GraphQlRequest, GraphQlTransport, GATEWAY_TYPE_GRAPHQL};
pub(crate) use mapper::map;
pub(crate) use network::{request, with_headers, GatewayTransport, HttpClient};
pub(crate) use rest::{RestTransport, GATEWAY_TYPE_REST};
