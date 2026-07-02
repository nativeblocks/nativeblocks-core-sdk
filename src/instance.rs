use crate::common::logger;
use crate::{config, frame, global_parameter, localization};

#[uniffi::export]
pub fn dispose_instance(instance_name: String) {
    frame::dispose(&instance_name);
    localization::dispose(&instance_name);
    config::remove(&instance_name);
    global_parameter::remove(&instance_name);
    logger::remove(&instance_name);
}

#[uniffi::export]
pub fn warmup_instance() {}

#[uniffi::export(async_runtime = "tokio")]
pub async fn warmup_instance_async() {
    tokio::task::yield_now().await;
}
