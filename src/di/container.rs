use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::library::cache::CacheProvider;
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
    cache: Arc<dyn CacheProvider>,
    logger: Arc<Mutex<NativeLoggerProvider>>,
    global_parameters: Arc<GlobalParameterProvider>,
    config_client: Mutex<Option<Arc<config::Client>>>,
    components: Mutex<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>,
}

impl Container {
    pub(crate) fn new(
        environment: NativeblocksEnvironment,
        sdk_config: SdkConfig,
        http: Arc<dyn HttpClient>,
        cache: Arc<dyn CacheProvider>,
    ) -> Self {
        let logger = logger::get_or_create(environment.instance_name());
        return Self {
            environment,
            sdk_config,
            http,
            cache,
            logger,
            global_parameters: global_parameter::build_provider(),
            config_client: Mutex::new(None),
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

    pub(crate) fn cache(&self) -> Arc<dyn CacheProvider> {
        return self.cache.clone();
    }

    pub(crate) fn logger(&self) -> Arc<Mutex<NativeLoggerProvider>> {
        return self.logger.clone();
    }

    pub(crate) fn global_parameters(&self) -> Arc<GlobalParameterProvider> {
        return self.global_parameters.clone();
    }

    pub(crate) fn config_client(&self) -> Arc<config::Client> {
        let mut guard = self.config_client.lock().unwrap();
        if let Some(existing) = guard.as_ref() {
            return existing.clone();
        }
        let client = build_client(
            self.environment.clone(),
            self.sdk_config.clone(),
            self.http.clone(),
            self.cache.clone(),
        );
        *guard = Some(client.clone());
        return client;
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
