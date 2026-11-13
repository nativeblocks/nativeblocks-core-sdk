use std::sync::{Arc, Mutex};

use tokio::sync::OnceCell;

use crate::feature::embedded::manifest::{self, EmbeddedManifest, MANIFEST_FILE};
use crate::feature::embedded::logging;
use crate::feature::embedded::reader::EmbeddedReader;
use crate::feature::scaffold::ScaffoldModel;
use crate::feature::{frame, localization, scaffold};
use crate::library::cache::CacheProvider;
use crate::library::cache::Caches;
use crate::library::environment::model::{NativeblocksEnvironment, SdkConfig};
use crate::library::result::ErrorModel;
use crate::plugin::logger::NativeLoggerProvider;

const MARKER_KEY: &str = "EMBEDDED_SEEDED";

pub(crate) enum SeedError {
    Invalid(String),
    Cache(ErrorModel),
}

pub(crate) struct Embedded {
    reader: Arc<dyn EmbeddedReader>,
    environment: NativeblocksEnvironment,
    sdk_config: SdkConfig,
    frames: Arc<dyn CacheProvider>,
    localizations: Arc<dyn CacheProvider>,
    scaffold: Arc<dyn CacheProvider>,
    marker: Arc<dyn CacheProvider>,
    logger: Arc<Mutex<NativeLoggerProvider>>,
    seeded: OnceCell<()>,
}

struct Seeded {
    frames: usize,
    localizations: usize,
}

impl Embedded {
    pub(crate) fn new(
        reader: Arc<dyn EmbeddedReader>,
        environment: NativeblocksEnvironment,
        sdk_config: SdkConfig,
        caches: &Caches,
        logger: Arc<Mutex<NativeLoggerProvider>>,
    ) -> Self {
        return Self {
            reader,
            environment,
            sdk_config,
            frames: caches.frames.clone(),
            localizations: caches.localizations.clone(),
            scaffold: caches.scaffold.clone(),
            marker: caches.embedded.clone(),
            logger,
            seeded: OnceCell::new(),
        };
    }

    pub(crate) async fn ready(&self) {
        self.seeded.get_or_init(|| self.seed()).await;
    }

    async fn seed(&self) {
        if self.environment.development_mode() {
            return;
        }
        if self.has_marker().await {
            return;
        }
        let seeded = match self.seed_from().await {
            Ok(seeded) => seeded,
            Err(error) => {
                logging::log_failure(&self.logger, &self.sdk_config, &error);
                return;
            }
        };
        if let Err(error) = self.write_marker().await {
            logging::log_failure(&self.logger, &self.sdk_config, &error);
            return;
        }
        if let Some(seeded) = seeded {
            logging::log_seeded(
                &self.logger,
                &self.sdk_config,
                seeded.frames,
                seeded.localizations,
            );
        }
    }

    async fn seed_from(&self) -> Result<Option<Seeded>, ErrorModel> {
        let Some(bytes) = self.read(MANIFEST_FILE).await else {
            return Ok(None);
        };
        let manifest = match manifest::parse(&bytes, &self.sdk_config.platform) {
            Ok(manifest) => manifest,
            Err(reason) => {
                self.skip(MANIFEST_FILE, &reason);
                return Ok(None);
            }
        };
        let frames = self.seed_frames(&manifest).await?;
        let localizations = self.seed_localizations(&manifest).await?;
        self.seed_scaffold(&manifest).await?;
        return Ok(Some(Seeded {
            frames,
            localizations,
        }));
    }

    async fn seed_frames(&self, manifest: &EmbeddedManifest) -> Result<usize, ErrorModel> {
        let mut count = 0;
        for entry in &manifest.frames {
            let Some(file) = self.read_entry(&entry.route, &entry.file).await else {
                continue;
            };
            let result =
                frame::seed_embedded(self.frames.as_ref(), &entry.route, &file, &entry.checksum)
                    .await;
            count += self.count(&entry.route, result)?;
        }
        return Ok(count);
    }

    async fn seed_localizations(&self, manifest: &EmbeddedManifest) -> Result<usize, ErrorModel> {
        let mut count = 0;
        for entry in &manifest.localizations {
            let Some(file) = self.read_entry(&entry.language_code, &entry.file).await else {
                continue;
            };
            let result = localization::seed_embedded(
                self.localizations.as_ref(),
                &entry.language_code,
                &file,
                &entry.checksum,
            )
            .await;
            count += self.count(&entry.language_code, result)?;
        }
        return Ok(count);
    }

    async fn seed_scaffold(&self, manifest: &EmbeddedManifest) -> Result<(), ErrorModel> {
        if manifest.frames.is_empty() {
            return Ok(());
        }
        let scaffold = ScaffoldModel {
            frames: manifest.frames.iter().map(manifest::to_frame_route).collect(),
        };
        let result = scaffold::seed_embedded(self.scaffold.as_ref(), &scaffold).await;
        self.count("scaffold", result)?;
        return Ok(());
    }

    async fn read_entry(&self, key: &str, path: &str) -> Option<Vec<u8>> {
        if !manifest::is_safe_path(path) {
            self.skip(key, &format!("invalid file path '{path}'"));
            return None;
        }
        let file = self.read(path).await;
        if file.is_none() {
            self.skip(key, &format!("file '{path}' is not in the app"));
        }
        return file;
    }

    fn count(&self, key: &str, result: Result<bool, SeedError>) -> Result<usize, ErrorModel> {
        return match result {
            Ok(true) => Ok(1),
            Ok(false) => Ok(0),
            Err(SeedError::Invalid(reason)) => {
                self.skip(key, &reason);
                Ok(0)
            }
            Err(SeedError::Cache(error)) => Err(error),
        };
    }

    fn skip(&self, key: &str, reason: &str) {
        logging::log_skipped(&self.logger, &self.sdk_config, key, reason);
    }

    async fn read(&self, path: &str) -> Option<Vec<u8>> {
        let reader = self.reader.clone();
        let path = format!("{}/{path}", self.environment.instance_name());
        return tokio::task::spawn_blocking(move || reader.read(path))
            .await
            .ok()
            .flatten();
    }

    async fn has_marker(&self) -> bool {
        return self
            .marker
            .has(MARKER_KEY.to_string())
            .await
            .unwrap_or(false);
    }

    async fn write_marker(&self) -> Result<(), ErrorModel> {
        self.marker
            .save(MARKER_KEY.to_string(), vec![1], None)
            .await?;
        return Ok(());
    }
}
