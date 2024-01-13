use crate::modules::music::Token;
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::fs::{self, File};
use std::io::Write;
use toml::de;

#[derive(Serialize, Deserialize, Default)]
pub struct Cache {
    pub modules: ModulesCache,
}

#[derive(thiserror::Error, Debug)]
pub enum CacheError {
    #[error("Invalid cache file: {0}")]
    FailedToParseToml(de::Error),
    #[error("Failed to open file to write to, path: {0}")]
    FailedToWriteFile(std::io::Error),
    #[error("Failed to convert cache to string for writing: {0}")]
    FailedToPrintCache(toml::ser::Error),
}

impl Cache {
    pub fn from_file(path: &str) -> Result<Self, CacheError> {
        match fs::read_to_string(path) {
            Ok(raw_cache) => toml::from_str(&raw_cache).map_err(CacheError::FailedToParseToml),
            Err(err) => {
                eprintln!(
                    "WARNING: Could not read cache file from `{}`: {}",
                    path, err
                );
                Ok(Self::default())
            }
        }
    }

    pub fn to_file(&self, path: &str) -> Result<(), CacheError> {
        let mut file = File::create(path).map_err(CacheError::FailedToWriteFile)?;
        let cache = toml::to_string(self).map_err(CacheError::FailedToPrintCache)?;
        file.write_all(&cache.into_bytes())
            .map_err(CacheError::FailedToWriteFile)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct ModulesCache {
    pub spotify: Option<SpotifyCache>,
}

#[derive(Serialize, Deserialize)]
pub struct SpotifyCache {
    pub token: Token,
}
