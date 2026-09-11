use crate::library::cache::CacheProvider;
use crate::library::cache::util::{
    HEADER_LEN, file_name, is_expired, now_millis, offload, read_expiry,
};
use crate::library::result::{ErrorModel, NBError, NBResult};
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

const ROOT_DIR: &str = "nativeblocks";
const FORMAT_DIR: &str = "v1";
const TEMP_SUFFIX: &str = ".tmp";

pub(crate) struct FileCacheProvider {
    cache_root_dir: PathBuf,
    seq: AtomicU64,
}

impl FileCacheProvider {
    pub(crate) fn new(cache_dir: &str, instance_name: &str, namespace: &str) -> NBResult<Self> {
        let cache_root_dir = Path::new(cache_dir)
            .join(ROOT_DIR)
            .join(FORMAT_DIR)
            .join(instance_name)
            .join(namespace);

        fs::create_dir_all(&cache_root_dir)
            .map_err(|error| ErrorModel::cache(error.to_string()))?;

        Self::delete_stale_cache(cache_dir);

        return Ok(Self {
            cache_root_dir,
            seq: AtomicU64::new(0),
        });
    }

    fn path_of(&self, key: &str) -> PathBuf {
        return self.cache_root_dir.join(file_name(key));
    }

    fn temp_path(&self, key: &str) -> PathBuf {
        return self.cache_root_dir.join(format!(
            "{}.{}.{}{TEMP_SUFFIX}",
            file_name(key),
            std::process::id(),
            self.seq.fetch_add(1, Ordering::Relaxed)
        ));
    }

    fn delete_stale_cache(cache_dir: &str) {
        let entries = fs::read_dir(Path::new(cache_dir).join(ROOT_DIR));
        if entries.is_err() {
            return;
        };
        for entry in entries.unwrap().flatten() {
            let is_current_format = entry.file_name() == FORMAT_DIR;
            if !is_current_format {
                let _ = fs::remove_dir_all(entry.path());
            }
        }
    }
}

#[async_trait::async_trait]
impl CacheProvider for FileCacheProvider {
    /// Writes one cache entry as a single file, atomically.
    ///
    /// Path: `<cache_dir>/nativeblocks/v1/<instance_name>/<32-hex FNV-1a-128 of key>`
    ///
    /// byte 0               8                       8 + N
    /// +-------------------+-----------------------+
    /// |  expiry  i64 LE   |  value  N bytes       |
    /// |  0 = never        |  data payload         |
    /// +-------------------+-----------------------+
    ///
    /// The record goes to `<hash>.<pid>.<seq>.tmp` in the same directory and is
    /// renamed onto `<hash>`, so a reader never sees a partial record. The temp
    /// file is removed if either step fails.
    async fn save(
        &self,
        key: String,
        value: Vec<u8>,
        ttl_millis: Option<i64>,
    ) -> Result<(), NBError> {
        let expiry = ttl_millis.map(|ttl| now_millis() + ttl).unwrap_or(0); // 0 = never expires

        let mut data = Vec::with_capacity(HEADER_LEN + value.len());
        data.extend_from_slice(&expiry.to_le_bytes());
        data.extend_from_slice(&value);

        let temp = self.temp_path(&key);
        let target = self.path_of(&key);

        return offload(move || {
            let written_file = fs::write(&temp, &data).and_then(|_| fs::rename(&temp, &target));
            return match written_file {
                Ok(_) => Ok(()),
                Err(error) => {
                    let _ = fs::remove_file(&temp);
                    return Err(ErrorModel::cache(error.to_string()));
                }
            };
        })
        .await
        .map_err(NBError::from);
    }

    async fn get(&self, key: String) -> Result<Option<Vec<u8>>, NBError> {
        let file_path = self.path_of(&key);
        return offload(move || {
            let Ok(data) = fs::read(&file_path) else {
                return Ok(None);
            };
            if data.len() < HEADER_LEN {
                let _ = fs::remove_file(&file_path);
                return Ok(None);
            }
            let expiry = read_expiry(&data[..HEADER_LEN]);
            if is_expired(expiry) {
                let _ = fs::remove_file(&file_path);
                return Ok(None);
            }
            let data_without_header = data[HEADER_LEN..].to_vec();
            return Ok(Some(data_without_header));
        })
        .await
        .map_err(NBError::from);
    }

    async fn remove(&self, key: String) -> Result<(), NBError> {
        let path = self.path_of(&key);
        return offload(move || {
            let _ = fs::remove_file(&path);
            return Ok(());
        })
        .await
        .map_err(NBError::from);
    }

    async fn has(&self, key: String) -> Result<bool, NBError> {
        let file_path = self.path_of(&key);
        return offload(move || {
            let file = File::open(&file_path);
            if file.is_err() {
                return Ok(false);
            }
            let header = {
                let mut buf = [0u8; HEADER_LEN];
                if file.unwrap().read_exact(&mut buf).is_err() {
                    let _ = fs::remove_file(&file_path);
                    return Ok(false);
                }
                buf
            };

            let expiry = read_expiry(&header);
            if is_expired(expiry) {
                let _ = fs::remove_file(&file_path);
                return Ok(false);
            }
            return Ok(true);
        })
        .await
        .map_err(NBError::from);
    }

    async fn clear(&self) -> Result<(), NBError> {
        let dir = self.cache_root_dir.clone();
        return offload(move || {
            let Ok(entries) = fs::read_dir(&dir) else {
                return Ok(());
            };
            for entry in entries.flatten() {
                if entry.path().is_file() {
                    let _ = fs::remove_file(entry.path());
                }
            }
            return Ok(());
        })
        .await
        .map_err(NBError::from);
    }
}
