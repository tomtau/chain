#![cfg(feature = "sled")]
use std::path::Path;

use failure::ResultExt;
use sled::{ConfigBuilder, Db};

use crate::storage::Storage;
use crate::{Error, ErrorKind};

/// Storage backed by Sled
pub struct SledStorage(Db);

impl SledStorage {
    /// Creates a new instance with specified path for data storage
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        Ok(Self(
            Db::start(ConfigBuilder::new().path(path).build())
                .context(ErrorKind::StorageInitializationError)?,
        ))
    }
}

impl Storage for SledStorage {
    fn clear(&self) -> Result<(), Error> {
        self.0.clear().context(ErrorKind::StorageError)?;
        Ok(())
    }

    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, Error> {
        let value = self.0.get(key).context(ErrorKind::StorageError)?;
        let value = value.map(|inner| inner.to_vec());

        Ok(value)
    }

    fn set(&self, key: &[u8], value: Vec<u8>) -> Result<Option<Vec<u8>>, Error> {
        let value = self.0.set(key, value).context(ErrorKind::StorageError)?;
        let value = value.map(|inner| inner.to_vec());

        Ok(value)
    }

    fn keys(&self) -> Result<Vec<Vec<u8>>, Error> {
        self.0
            .iter()
            .keys()
            .map(|key| Ok(key.context(ErrorKind::StorageError)?))
            .collect()
    }

    fn contains_key(&self, key: &[u8]) -> Result<bool, Error> {
        Ok(self.0.contains_key(key).context(ErrorKind::StorageError)?)
    }
}
