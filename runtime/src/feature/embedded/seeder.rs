use std::sync::{Arc, Mutex};

use tokio::sync::Mutex as AsyncMutex;

use crate::feature::embedded::logging::SeedLog;
use crate::feature::embedded::manifest::{self, EmbeddedManifest, MANIFEST_FILE};
use crate::feature::embedded::reader::EmbeddedReader;
use crate::feature::scaffold::ScaffoldModel;
use crate::feature::{frame, localization, scaffold};
use crate::library::cache::{CacheProvider, Caches};
use crate::library::environment::model::{NativeblocksEnvironment, SdkConfig};
use crate::library::result::ErrorModel;
use crate::plugin::config;
use crate::plugin::logger::NativeLoggerProvider;

const MARKER_KEY: &str = "EMBEDDED_SEEDED";

pub(crate) enum SeedError {
    Invalid(String),
    Cache(ErrorModel),
}

pub(crate) struct Embedded {
    reader: Arc<dyn EmbeddedReader>,
    development_mode: bool,
    platform: String,
    frames: Arc<dyn CacheProvider>,
    localizations: Arc<dyn CacheProvider>,
    scaffold: Arc<dyn CacheProvider>,
    marker: Arc<dyn CacheProvider>,
    config: Arc<config::Client>,
    log: SeedLog,
    done: AsyncMutex<bool>,
}

impl Embedded {
    pub(crate) fn new(
        reader: Arc<dyn EmbeddedReader>,
        environment: NativeblocksEnvironment,
        sdk_config: SdkConfig,
        caches: &Caches,
        config: Arc<config::Client>,
        logger: Arc<Mutex<NativeLoggerProvider>>,
    ) -> Self {
        return Self {
            reader,
            development_mode: environment.development_mode(),
            platform: sdk_config.platform.clone(),
            frames: caches.frames.clone(),
            localizations: caches.localizations.clone(),
            scaffold: caches.scaffold.clone(),
            marker: caches.embedded.clone(),
            config,
            log: SeedLog::new(logger, sdk_config),
            done: AsyncMutex::new(false),
        };
    }

    pub(crate) async fn ready(&self) {
        let mut done = self.done.lock().await;
        if !*done {
            *done = self.seed_once().await;
        }
    }

    async fn seed_once(&self) -> bool {
        if self.development_mode || self.has_marker().await {
            return true;
        }
        let Ok(project_id) = self.config.project_id().await else {
            return false;
        };
        if let Err(error) = self.seed(&project_id).await {
            self.log.failed(&error);
        }
        return true;
    }

    async fn seed(&self, project_id: &str) -> Result<(), ErrorModel> {
        if let Some(manifest) = self.manifest(project_id).await {
            let frames = self.seed_frames(project_id, &manifest).await?;
            let localizations = self.seed_localizations(project_id, &manifest).await?;
            self.seed_scaffold(&manifest).await?;
            self.log.seeded(frames, localizations);
        }
        return self.write_marker().await;
    }

    async fn manifest(&self, project_id: &str) -> Option<EmbeddedManifest> {
        if !manifest::is_safe_name(project_id) {
            self.log
                .skipped(MANIFEST_FILE, &format!("invalid project id '{project_id}'"));
            return None;
        }
        let bytes = self.read(project_id, MANIFEST_FILE).await?;
        return match manifest::parse(&bytes, project_id, &self.platform) {
            Ok(manifest) => Some(manifest),
            Err(reason) => {
                self.log.skipped(MANIFEST_FILE, &reason);
                None
            }
        };
    }

    async fn seed_frames(
        &self,
        project_id: &str,
        manifest: &EmbeddedManifest,
    ) -> Result<usize, ErrorModel> {
        let mut seeded = 0;
        for entry in &manifest.frames {
            let Some(file) = self.read_entry(project_id, &entry.route, &entry.file).await else {
                continue;
            };
            let result =
                frame::seed_embedded(self.frames.as_ref(), &entry.route, &file, &entry.checksum)
                    .await;
            seeded += self.applied(&entry.route, result)?;
        }
        return Ok(seeded);
    }

    async fn seed_localizations(
        &self,
        project_id: &str,
        manifest: &EmbeddedManifest,
    ) -> Result<usize, ErrorModel> {
        let mut seeded = 0;
        for entry in &manifest.localizations {
            let key = &entry.language_code;
            let Some(file) = self.read_entry(project_id, key, &entry.file).await else {
                continue;
            };
            let result = localization::seed_embedded(
                self.localizations.as_ref(),
                key,
                &file,
                &entry.checksum,
            )
            .await;
            seeded += self.applied(key, result)?;
        }
        return Ok(seeded);
    }

    async fn seed_scaffold(&self, manifest: &EmbeddedManifest) -> Result<(), ErrorModel> {
        if manifest.frames.is_empty() {
            return Ok(());
        }
        let scaffold = ScaffoldModel {
            frames: manifest
                .frames
                .iter()
                .map(manifest::to_frame_route)
                .collect(),
        };
        let result = scaffold::seed_embedded(self.scaffold.as_ref(), &scaffold).await;
        self.applied("scaffold", result)?;
        return Ok(());
    }

    fn applied(&self, key: &str, result: Result<bool, SeedError>) -> Result<usize, ErrorModel> {
        return match result {
            Ok(written) => Ok(written as usize),
            Err(SeedError::Invalid(reason)) => {
                self.log.skipped(key, &reason);
                Ok(0)
            }
            Err(SeedError::Cache(error)) => Err(error),
        };
    }

    async fn read_entry(&self, project_id: &str, key: &str, path: &str) -> Option<Vec<u8>> {
        if !manifest::is_safe_path(path) {
            self.log
                .skipped(key, &format!("invalid file path '{path}'"));
            return None;
        }
        let file = self.read(project_id, path).await;
        if file.is_none() {
            self.log
                .skipped(key, &format!("file '{path}' is not in the app"));
        }
        return file;
    }

    async fn read(&self, project_id: &str, path: &str) -> Option<Vec<u8>> {
        let reader = self.reader.clone();
        let path = format!("{project_id}/{path}");
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
