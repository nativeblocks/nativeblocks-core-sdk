// These clippy lints flag deliberate, codebase-wide patterns rather than defects:
#![allow(clippy::needless_return)] // explicit `return` is the house style
#![allow(clippy::too_many_arguments)] // data-layer fns thread request context explicitly
#![allow(clippy::collapsible_if)] // nested `if let` kept; let-chains are still unstable
#![allow(clippy::match_single_binding)] // versioned cache-migration dispatch placeholder
#![allow(clippy::wildcard_in_or_patterns)] // explicit default branch documents intent

uniffi::setup_scaffolding!("NativeblocksRuntime");

pub(crate) mod di;
pub mod feature;
pub mod library;
pub mod plugin;

use std::sync::Arc;

use crate::di::Container;
use crate::feature::experiment::ExperimentClient;
use crate::feature::frame::FrameClient;
use crate::feature::localization::LocalizationClient;
use crate::feature::scaffold::ScaffoldClient;
use crate::feature::{experiment, frame, localization, scaffold};
use crate::library::cache::CacheProvider;
use crate::library::environment::model::{NativeblocksEnvironment, SdkConfig};
use crate::library::net::network::HttpClient;
use crate::library::result::NBError;
use crate::plugin::global_parameter;
use crate::plugin::global_parameter::GlobalParameterClient;
use crate::plugin::logger;

#[derive(uniffi::Object)]
pub struct NativeblocksRuntime {
    container: Arc<Container>,
}

#[uniffi::export]
impl NativeblocksRuntime {
    #[uniffi::constructor]
    pub fn new(
        environment: NativeblocksEnvironment,
        config: SdkConfig,
        http: Arc<dyn HttpClient>,
        cache: Arc<dyn CacheProvider>,
    ) -> Result<Arc<Self>, NBError> {
        let container = di::get_or_create(&environment, &config, http, cache)?;
        return Ok(Arc::new(Self { container }));
    }

    pub fn frame_client(&self) -> Arc<FrameClient> {
        return frame::get_or_create_client(&self.container);
    }

    pub fn scaffold_client(&self) -> Arc<ScaffoldClient> {
        return scaffold::get_or_create_client(&self.container);
    }

    pub fn experiment_client(&self) -> Arc<ExperimentClient> {
        return experiment::get_or_create_client(&self.container);
    }

    pub fn localization_client(&self) -> Arc<LocalizationClient> {
        return localization::get_or_create_client(&self.container);
    }

    pub fn global_parameter_client(&self) -> Arc<GlobalParameterClient> {
        return global_parameter::get_or_create_client(&self.container);
    }
}

#[uniffi::export]
pub fn dispose_instance(instance_name: String) {
    if let Some(container) = di::remove(&instance_name) {
        container.dispose();
    }
    logger::remove(&instance_name);
}
