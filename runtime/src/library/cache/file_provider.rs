use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use crate::library::cache::CacheProvider;
use crate::library::cache::util::{self, HEADER_LEN, file_name, is_expired, read_expiry};
use crate::library::environment::is_valid_instance_name;
use crate::library::result::{ErrorModel, NBError, NBResult};

const ROOT_DIR: &str = "nativeblocks";
const FORMAT_DIR: &str = "v1";
const TEMP_SUFFIX: &str = ".tmp";

pub(crate) struct FileCacheProvider {
    cache_root_dir: PathBuf,
    seq: AtomicU64,
}

impl FileCacheProvider {
    pub(crate) fn new(cache_dir: &str, instance_name: &str) -> NBResult<Self> {
        if !is_valid_instance_name(instance_name) {
            return Err(ErrorModel::cache(format!(
                "Please make sure the instance name '{instance_name}' contains valid characters: A-Z, a-z, 0-9, _ or -"
            )));
        }

        let cache_root_dir = Path::new(cache_dir)
            .join(ROOT_DIR)
            .join(FORMAT_DIR)
            .join(instance_name);
        fs::create_dir_all(&cache_root_dir)
            .map_err(|error| ErrorModel::cache(error.to_string()))?;

        if let Ok(entries) = fs::read_dir(Path::new(cache_dir).join(ROOT_DIR)) {
            for entry in entries.flatten() {
                if entry.file_name() != FORMAT_DIR {
                    let _ = fs::remove_dir_all(entry.path());
                }
            }
        }

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
}

#[async_trait::async_trait]
impl CacheProvider for FileCacheProvider {
    async fn save(
        &self,
        key: String,
        value: Vec<u8>,
        ttl_millis: Option<i64>,
    ) -> Result<(), NBError> {
        let expiry = ttl_millis.map(|ttl| util::now_millis() + ttl).unwrap_or(0);

        let mut record = Vec::with_capacity(HEADER_LEN + value.len());
        record.extend_from_slice(&expiry.to_le_bytes());
        record.extend_from_slice(&value);

        let target = self.path_of(&key);
        let temp = self.temp_path(&key);
        return Ok(util::offload(move || {
            let written = fs::write(&temp, &record).and_then(|_| fs::rename(&temp, &target));
            if let Err(error) = written {
                let _ = fs::remove_file(&temp);
                return Err(ErrorModel::cache(error.to_string()));
            }
            return Ok(());
        })
        .await?);
    }

    async fn get(&self, key: String) -> Result<Option<Vec<u8>>, NBError> {
        let path = self.path_of(&key);
        return Ok(util::offload(move || {
            let Ok(record) = fs::read(&path) else {
                return Ok(None);
            };
            if record.len() < HEADER_LEN {
                let _ = fs::remove_file(&path);
                return Ok(None);
            }
            if is_expired(read_expiry(&record[..HEADER_LEN])) {
                let _ = fs::remove_file(&path);
                return Ok(None);
            }
            return Ok(Some(record[HEADER_LEN..].to_vec()));
        })
        .await?);
    }

    async fn remove(&self, key: String) -> Result<(), NBError> {
        let path = self.path_of(&key);
        return Ok(util::offload(move || {
            let _ = fs::remove_file(&path);
            return Ok(());
        })
        .await?);
    }

    async fn has(&self, key: String) -> Result<bool, NBError> {
        let path = self.path_of(&key);
        return Ok(util::offload(move || {
            // Only the header is read, so probing a cached frame does not pull
            // its payload off disk.
            let Ok(mut file) = File::open(&path) else {
                return Ok(false);
            };
            let mut header = [0u8; HEADER_LEN];
            if file.read_exact(&mut header).is_err() {
                let _ = fs::remove_file(&path);
                return Ok(false);
            }
            if is_expired(read_expiry(&header)) {
                let _ = fs::remove_file(&path);
                return Ok(false);
            }
            return Ok(true);
        })
        .await?);
    }
}
