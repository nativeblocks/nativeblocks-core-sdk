mod data;
mod di;
mod domain;
mod presenter;

pub(crate) use di::get_or_create;
pub(crate) use presenter::client::Client;
