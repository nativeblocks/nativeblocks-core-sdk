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
use crate::frame::domain::model::NativeFrameModel;
use crate::frame::domain::repository::{FrameRepository, FrameUpdate};

pub(crate) struct CloudFrameRepository {
    http: Arc<dyn HttpClient>,
    environment: NativeblocksEnvironment,
    sdk_config: SdkConfig,
    cache: Arc<dyn CacheProvider>,
    config_client: Arc<config::Client>,
    logger: Arc<Mutex<NativeLoggerProvider>>,
    channels: Mutex<HashMap<String, watch::Sender<FrameUpdate>>>,
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
        };
    }

    fn read(&self, route: &str) -> FrameUpdate {
        return db_source::get_frame(self.cache.as_ref(), route, self.environment.development_mode()).map(Arc::new);
    }

    fn send(&self, route: &str, value: FrameUpdate) {
        let mut channels = self.channels.lock().unwrap();
        if let Some(sender) = channels.get(route) {
            if sender.send(value).is_err() {
                channels.remove(route);
            }
        }
    }

    fn channel_has_frame(&self, route: &str) -> bool {
        let channels = self.channels.lock().unwrap();
        return channels
            .get(route)
            .map(|sender| sender.borrow().is_ok())
            .unwrap_or(false);
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
        let result = self.fetch(route, parameters).await;
        match &result {
            Ok(_) => logging::log_sync_success(&self.logger, &self.sdk_config, route),
            Err(error) => logging::log_sync_failure(&self.logger, &self.sdk_config, route, error),
        }

        match &result {
            Ok(frame) => {
                // Dev: sync ui each time the db updated.
                // Prod: emit only when subscribers have no frame yet (first
                // download, or the read failed when they attached); otherwise
                // the db updates silently and the next visit renders the new
                // frame. The fetched frame is already in hand, so send it
                // directly instead of re-reading it from the db.
                if self.environment.development_mode() || !self.channel_has_frame(route) {
                    self.send(route, Ok(Arc::new(frame.clone())));
                }
            }
            Err(error) => {
                // The failure must reach the UI when nothing is rendered yet,
                // otherwise the frame stays on Loading forever. When a frame is
                // already showing, a background refresh failure stays silent.
                if !self.channel_has_frame(route) {
                    self.send(route, Err(error.clone()));
                }
            }
        }
        return result.map(|_| ());
    }

    fn get(&self, route: &str) -> watch::Receiver<FrameUpdate> {
        {
            let mut channels = self.channels.lock().unwrap();
            channels.retain(|_, sender| !sender.is_closed());
            if let Some(sender) = channels.get(route) {
                return sender.subscribe();
            }
        }
        let initial = self.read(route);
        let mut channels = self.channels.lock().unwrap();
        return channels
            .entry(route.to_string())
            .or_insert_with(|| watch::channel(initial).0)
            .subscribe();
    }

    async fn clear(&self, route: &str) -> NBResult<()> {
        return db_source::clear(self.cache.as_ref(), route).await;
    }

    async fn clear_all(&self, routes: &[String]) -> NBResult<()> {
        return db_source::clear_all(self.cache.as_ref(), routes).await;
    }
}
