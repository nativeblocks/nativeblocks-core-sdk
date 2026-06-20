use super::*;

#[test]
fn round_trips_typed_values() {
    let cache = SqliteCacheProvider::in_memory().unwrap();
    cache.save_string("s", "hello", None).unwrap();
    cache.save_bool("b", true, None).unwrap();
    cache.save_i64("i", 42, None).unwrap();
    cache.save_f64("f", 3.5, None).unwrap();

    assert_eq!(cache.get_string("s", "x").unwrap(), "hello");
    assert!(cache.get_bool("b", false).unwrap());
    assert_eq!(cache.get_i64("i", 0).unwrap(), 42);
    assert_eq!(cache.get_f64("f", 0.0).unwrap(), 3.5);
}

#[test]
fn returns_default_when_missing() {
    let cache = SqliteCacheProvider::in_memory().unwrap();
    assert_eq!(cache.get_string("nope", "fallback").unwrap(), "fallback");
    assert!(!cache.has("nope").unwrap());
}

#[test]
fn expired_entry_is_evicted_live_entry_kept() {
    let cache = SqliteCacheProvider::in_memory().unwrap();
    cache
        .save_string("short", "v", Some(Duration::from_millis(1)))
        .unwrap();
    std::thread::sleep(Duration::from_millis(5));
    assert_eq!(cache.get_string("short", "default").unwrap(), "default");
    assert!(!cache.has("short").unwrap());

    cache
        .save_string("long", "v2", Some(Duration::from_secs(3600)))
        .unwrap();
    assert_eq!(cache.get_string("long", "default").unwrap(), "v2");
    assert!(cache.has("long").unwrap());
}

#[test]
fn remove_and_clear() {
    let cache = SqliteCacheProvider::in_memory().unwrap();
    cache.save_string("a", "1", None).unwrap();
    cache.save_string("b", "2", None).unwrap();
    cache.remove("a").unwrap();
    assert!(!cache.has("a").unwrap());
    assert!(cache.has("b").unwrap());
    cache.clear().unwrap();
    assert!(!cache.has("b").unwrap());
}
