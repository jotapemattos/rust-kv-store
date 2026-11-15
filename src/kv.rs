use crate::Result;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufWriter, Write},
    path::PathBuf,
};

/// The key-value store.
pub struct KvStore {
    log: BufWriter<File>,
    index: HashMap<String, String>,
}

impl KvStore {
    /// Open the KvStore at a given path.
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let path: PathBuf = path.into();
        std::fs::create_dir_all(&path)?;
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path.join("kvs.log"))?;

        Ok(KvStore {
            log: BufWriter::new(file),
            index: HashMap::new(),
        })
    }

    /// Set the value of a string key to a string.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let cmd = Command::Set {
            key: key.clone(),
            value: value.clone(),
        };

        serde_json::to_writer(&mut self.log, &cmd)?;
        self.log.flush()?;
        self.index.insert(key, value);

        Ok(())
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

#[derive(Serialize, Deserialize, Debug)]
enum Command {
    Set { key: String, value: String },
    Rm { key: String },
}
