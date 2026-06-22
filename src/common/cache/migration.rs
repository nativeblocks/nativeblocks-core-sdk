use crate::common::cache::CacheProvider;
use crate::common::result::NBResult;

const VERSION_KEY: &str = "native_cache_version";

const CACHE_VERSION: u32 = 1;

pub(crate) fn run_migrations(cache: &dyn CacheProvider, preserve_keys: &[&str]) -> NBResult<()> {
    match read_version(cache)? {
        Some(version) if version == CACHE_VERSION => return Ok(()),
        None => {}
        Some(from) => apply(cache, from, preserve_keys)?,
    }
    write_version(cache, CACHE_VERSION)?;
    return Ok(());
}

fn apply(cache: &dyn CacheProvider, from: u32, preserve_keys: &[&str]) -> NBResult<()> {
    if from > CACHE_VERSION {
        return invalidate(cache, preserve_keys);
    }
    for target in (from + 1)..=CACHE_VERSION {
        match target {
            _ => invalidate(cache, preserve_keys)?,
        }
    }
    return Ok(());
}

fn invalidate(cache: &dyn CacheProvider, preserve_keys: &[&str]) -> NBResult<()> {
    let preserved: Vec<(String, Vec<u8>)> = preserve_keys
        .iter()
        .filter_map(|key| {
            cache
                .get_bytes(key.to_string())
                .ok()
                .flatten()
                .map(|value| (key.to_string(), value))
        })
        .collect();
    cache.clear()?;
    for (key, value) in preserved {
        cache.save_bytes(key, value, None)?;
    }
    return Ok(());
}

fn read_version(cache: &dyn CacheProvider) -> NBResult<Option<u32>> {
    let bytes = cache.get_bytes(VERSION_KEY.to_string())?;
    return Ok(bytes
        .and_then(|b| <[u8; 4]>::try_from(b).ok())
        .map(u32::from_le_bytes));
}

fn write_version(cache: &dyn CacheProvider, version: u32) -> NBResult<()> {
    cache.save_bytes(VERSION_KEY.to_string(), version.to_le_bytes().to_vec(), None)?;
    return Ok(());
}
