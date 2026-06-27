use std::{
    fs::{self, read_to_string},
    path::Path,
};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use toml::from_str;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse config file: {0}")]
    Parse(#[from] toml::de::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Use `0.0.0.0` to bind on all interfaces.
    pub bind: String,

    /// Maximum number of players allowed on the server.
    pub max_players: usize,

    /// The message of the day (MOTD) displayed in the server list.
    pub motd: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bind: "0.0.0.0:25565".to_string(),
            max_players: 20,
            motd: "Welcome to Yoki server!".to_string(),
        }
    }
}

impl Config {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let path = path.as_ref();
        if !path.exists() {
            let config = Self::default();
            config.save(path)?;
            return Ok(config);
        }

        let raw = read_to_string(path)?;
        let config: Self = from_str(&raw)?;
        Ok(config)
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), ConfigError> {
        let raw = toml::to_string_pretty(self).expect("Config serialization is infallible");
        fs::write(path, raw)?;
        Ok(())
    }
}
