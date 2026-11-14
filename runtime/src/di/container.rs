use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::feature::embedded::{Embedded, EmbeddedReader};
use crate::library::cache::Caches;
use crate::library::environment::model::{NativeblocksEnvironment, SdkConfig};
use crate::library::net::network::HttpClient;
use crate::plugin::config::build_client;
use crate::plugin::global_parameter::GlobalParameterProvider;
use crate::plugin::logger::{self, NativeLoggerProvider};
use crate::plugin::{config, global_parameter};

pub(crate) struct Container {
    environment: NativeblocksEnvironment,
    sdk_config: SdkConfig,
    http: Arc<dyn HttpClient>,
    caches: Caches,
    embedded: Arc<Embedded>,
    logger: Arc<Mutex<NativeLoggerProvider>>,
    global_parameters: Arc<GlobalParameterProvider>,
    config_client: Arc<config::Client>,
    components: Mutex<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>,
}

impl Container {
    pub(crate) fn new(
        environment: NativeblocksEnvironment,
        sdk_config: SdkConfig,
        http: Arc<dyn HttpClient>,
        caches: Caches,
        embedded_reader: Arc<dyn EmbeddedReader>,
    ) -> Self {
        let logger = logger::get_or_create(environment.instance_name());
        let config_client = build_client(
            environment.clone(),
            sdk_config.clone(),
            http.clone(),
            caches.config.clone(),
        );
        let embedded = Arc::new(Embedded::new(
            embedded_reader,
            environment.clone(),
            sdk_config.clone(),
            &caches,
            config_client.clone(),
            logger.clone(),
        ));
        return Self {
            environment,
            sdk_config,
            http,
            caches,
            embedded,
            logger,
            global_parameters: global_parameter::build_provider(),
            config_client,
            components: Mutex::new(HashMap::new()),
        };
    }

    pub(crate) fn environment(&self) -> &NativeblocksEnvironment {
        return &self.environment;
    }

    pub(crate) fn sdk_config(&self) -> &SdkConfig {
        return &self.sdk_config;
    }

    pub(crate) fn http(&self) -> Arc<dyn HttpClient> {
        return self.http.clone();
    }

    pub(crate) fn caches(&self) -> &Caches {
        return &self.caches;
    }

    pub(crate) fn embedded(&self) -> Arc<Embedded> {
        return self.embedded.clone();
    }

    pub(crate) fn logger(&self) -> Arc<Mutex<NativeLoggerProvider>> {
        return self.logger.clone();
    }

    pub(crate) fn global_parameters(&self) -> Arc<GlobalParameterProvider> {
        return self.global_parameters.clone();
    }

    pub(crate) fn config_client(&self) -> Arc<config::Client> {
        return self.config_client.clone();
    }

    pub(crate) fn component<T>(&self, build: impl FnOnce() -> Arc<T>) -> Arc<T>
    where
        T: Any + Send + Sync,
    {
        let mut components = self.components.lock().expect("di container poisoned");
        if let Some(existing) = components.get(&TypeId::of::<T>()) {
            return existing
                .clone()
                .downcast::<T>()
                .expect("di component type mismatch");
        }
        let value = build();
        components.insert(TypeId::of::<T>(), value.clone());
        return value;
    }

    pub(crate) fn dispose(&self) {
        self.components
            .lock()
            .expect("di container poisoned")
            .clear();
    }
}
