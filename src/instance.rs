use crate::common::logger;
use crate::{di, frame, localization};

#[uniffi::export]
pub fn dispose_instance(instance_name: String) {
    frame::dispose(&instance_name);
    localization::dispose(&instance_name);
    di::remove(&instance_name);
    logger::remove(&instance_name);
}

#[uniffi::export]
pub fn warmup_instance() {}

#[uniffi::export(async_runtime = "tokio")]
pub async fn warmup_instance_async() {
    tokio::task::yield_now().await;
}
