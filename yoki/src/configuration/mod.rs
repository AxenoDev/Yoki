use std::{
    fs::{read_to_string, write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::configuration::{motd::MotdConfig, server::ServerConfig};

pub mod motd;
pub mod server;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse config file: {0}")]
    Parse(#[from] toml::de::Error),
}

#[derive(Debug, Default)]
pub struct Configuration {
    pub server: ServerConfig,
    pub motd: MotdConfig,
}

impl Configuration {
    pub fn load(dir: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let dir = dir.as_ref();
        Ok(Self {
            server: ServerConfig::load_from_dir(dir)?,
            motd: MotdConfig::load_from_dir(dir)?,
        })
    }
}

pub trait Config: Default + Serialize + for<'de> Deserialize<'de> {
    const FILE_NAME: &'static str;

    fn load_from_dir(dir: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let path: PathBuf = dir.as_ref().join(Self::FILE_NAME);

        if !path.exists() {
            std::fs::create_dir_all(dir.as_ref())?;
            let config = Self::default();
            config.save(&path)?;
            return Ok(config);
        }

        let raw = read_to_string(&path)?;
        Ok(toml::from_str(&raw)?)
    }

    fn save(&self, path: impl AsRef<Path>) -> Result<(), ConfigError> {
        let raw = toml::to_string_pretty(self).expect("Config serialization is infallible");
        write(path.as_ref(), raw)?;
        Ok(())
    }
}
