use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tokio::sync::watch;

use crate::common::cache::CacheProvider;
use crate::common::environment::{NativeblocksEnvironment, SdkConfig};
use crate::common::logger::NativeLoggerProvider;
use crate::common::net::HttpClient;
use crate::common::result::NBResult;
use crate::config;
use crate::frame::data::key::{GATEWAY_FRAME, GATEWAY_FRAME_PRODUCTION, GATEWAY_FRAME_PRODUCTION_CHECKSUM};
use crate::frame::data::logging;
use crate::frame::data::cloud_source;
use crate::frame::data::db_source;
use crate::frame::data::memory_source::MemoryFrameSource;
use crate::frame::domain::model::NativeFrameModel;
use crate::frame::domain::repository::FrameRepository;

type FrameUpdate = NBResult<NativeFrameModel>;

pub(crate) struct CloudFrameRepository {
    http: Arc<dyn HttpClient>,
    environment: NativeblocksEnvironment,
    sdk_config: SdkConfig,
    cache: Arc<dyn CacheProvider>,
    config_client: Arc<config::Client>,
    logger: Arc<Mutex<NativeLoggerProvider>>,
    channels: Mutex<HashMap<String, watch::Sender<FrameUpdate>>>,
    memory: MemoryFrameSource,
}

impl CloudFrameRepository {
    pub(crate) fn new(
        http: Arc<dyn HttpClient>,
        environment: NativeblocksEnvironment,
        sdk_config: SdkConfig,
        cache: Arc<dyn CacheProvider>,
        config_client: Arc<config::Client>,
        logger: Arc<Mutex<NativeLoggerProvider>>,
    ) -> Self {
        return Self {
            http,
            environment,
            sdk_config,
            cache,
            config_client,
            logger,
            channels: Mutex::new(HashMap::new()),
            memory: MemoryFrameSource::new(),
        };
    }

    fn read(&self, route: &str) -> FrameUpdate {
        let cached = self.memory.get_frame(route);
        if let Some(frame) = cached {
            return Ok(frame);
        }

        let from_db = db_source::get_frame(self.cache.as_ref(), route, self.environment.development_mode());
        if let Ok(frame) = &from_db {
            self.memory.save_frame(route, frame.clone());
        }
        return from_db;
    }

    fn emit(&self, route: &str) {
        let value = self.read(route);
        let mut channels = self.channels.lock().unwrap();
        if let Some(sender) = channels.get(route) {
            if sender.send(value).is_err() {
                channels.remove(route);
            }
        }
    }

    async fn fetch(
        &self,
        route: &str,
        parameters: &HashMap<String, String>,
    ) -> NBResult<NativeFrameModel> {
        let frame = self.config_client.gateway(GATEWAY_FRAME).await?;
        let production = self.config_client.gateway(GATEWAY_FRAME_PRODUCTION).await?;
        let checksum = self.config_client.gateway(GATEWAY_FRAME_PRODUCTION_CHECKSUM).await?;
        return cloud_source::sync_cloud(
            self.http.as_ref(),
            &self.environment,
            &self.sdk_config,
            self.cache.as_ref(),
            frame.gateway,
            production.gateway,
            checksum.gateway,
            &frame.endpoint,
            &frame.install_id,
            route,
            parameters,
        )
        .await;
    }
}

#[async_trait::async_trait]
impl FrameRepository for CloudFrameRepository {
    async fn sync(&self, route: &str, parameters: &HashMap<String, String>) -> NBResult<()> {
        let had_cached_frame = self.read(route).is_ok();

        let result = self.fetch(route, parameters).await;
        match &result {
            Ok(_) => logging::log_sync_success(&self.logger, &self.sdk_config, route),
            Err(error) => logging::log_sync_failure(&self.logger, &self.sdk_config, route, error),
        }

        // fetch already wrote the DB; keep the memory source in step with it.
        if let Ok(frame) = &result {
            self.memory.save_frame(route, frame.clone());
        }

        if result.is_ok() {
            if self.environment.development_mode() {
                // Dev: sync ui in dev mode each time the db updated.
                self.emit(route);
            } else if !had_cached_frame {
                // Prod: first download of this route
                self.emit(route);
            } else {
                // Prod: cache already served the UI: the DB updates silently
                // and the next visit renders the new frame.
            }
        }
        return result.map(|_| ());
    }

    fn get(&self, route: &str) -> watch::Receiver<FrameUpdate> {
        let initial = self.read(route);
        let mut channels = self.channels.lock().unwrap();
        channels.retain(|_, sender| !sender.is_closed());
        return channels
            .entry(route.to_string())
            .or_insert_with(|| watch::channel(initial).0)
            .subscribe();
    }

    async fn clear(&self, route: &str) -> NBResult<()> {
        self.memory.clear(route);
        return db_source::clear(self.cache.as_ref(), route).await;
    }

    async fn clear_all(&self, routes: &[String]) -> NBResult<()> {
        self.memory.clear_all(routes);
        return db_source::clear_all(self.cache.as_ref(), routes).await;
    }
}
