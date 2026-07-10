use crate::common::logger;
use crate::{di, frame, localization};

#[uniffi::export]
pub fn dispose_instance(instance_name: String) {
    frame::dispose(&instance_name);
    localization::dispose(&instance_name);
    // Drops the instance's whole DI container (repositories, config client,
    // global parameters). Live clients holding the Arc keep working until
    // released; the name resolves to a fresh graph afterwards.
    di::remove(&instance_name);
    // The logger registry is the one name-keyed map outside the container:
    // hosts may register loggers before any client (and thus container) exists.
    logger::remove(&instance_name);
}

#[uniffi::export]
pub fn warmup_instance() {}

#[uniffi::export(async_runtime = "tokio")]
pub async fn warmup_instance_async() {
    tokio::task::yield_now().await;
}
