use crate::Result;
use std::path::PathBuf;

/// The key-value store.
pub struct KvStore {}

impl KvStore {
    /// Open the KvStore at a given path.
    pub fn open(_path: impl Into<PathBuf>) -> Result<KvStore> {
        todo!()
    }

    /// Set the value of a string key to a string.
    pub fn set(&mut self, _key: String, _value: String) -> Result<()> {
        todo!()
    }

    /// Get the string value of a string key. Returns `None` if the key doesn't exist.
    pub fn get(&mut self, _key: String) -> Result<Option<String>> {
        todo!()
    }

    /// Remove a given key. Errors if the key doesn't exist.
    pub fn remove(&mut self, _key: String) -> Result<()> {
        todo!()
    }
}
