#[uniffi::export(with_foreign)]
pub trait EmbeddedReader: Send + Sync {
    fn read(&self, path: String) -> Option<Vec<u8>>;
}
