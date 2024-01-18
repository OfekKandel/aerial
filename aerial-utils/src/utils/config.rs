use serde::Deserialize;
use std::default::Default;
use std::fs;
use toml::de;

#[derive(Deserialize, Default)]
pub struct Config {
    pub modules: ModulesConfig,
}

#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("Failed to parse config file: {0}")]
    FailedToParseToml(de::Error),
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, ConfigError> {
        match fs::read_to_string(path) {
            Ok(raw_config) => toml::from_str(&raw_config).map_err(ConfigError::FailedToParseToml),
            Err(err) => {
                eprintln!("WARNING: Could not read config file from `{}`: {}", path, err);
                Ok(Self::default())
            }
        }
    }
}

#[derive(Deserialize, Default)]
pub struct ModulesConfig {
    pub spotify: Option<SpotifyConfig>,
}

#[derive(Deserialize)]
pub struct SpotifyConfig {
    // TODO: Add option to not set these in the config and give them at runtime instead
    pub client_id: String,
    pub client_secret: String,
}
