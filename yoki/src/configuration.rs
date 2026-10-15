use serde::{Deserialize, Serialize};
use std::{
    fs::{self},
    path::Path,
};
use thiserror::Error;

pub const PROPERTIES_FILE_NAME: &str = "server.properties";

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse config file: {0}")]
    Parse(#[from] serde_java_properties::de::Error),
    #[error("Failed to serialize server.properties")]
    Serialize(#[from] serde_java_properties::ser::Error),
    #[error("Failed to load registry data: {0}")]
    Registry(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct ServerProperties {
    pub server_ip: String,
    pub server_port: u16,
    pub max_players: usize,
    pub motd: String,
    pub online_mode: bool,
}

impl Default for ServerProperties {
    fn default() -> Self {
        Self {
            server_ip: "0.0.0.0".to_string(),
            server_port: 25565,
            max_players: 20,
            motd: "Welcome to Yoki server!".to_string(),
            online_mode: true,
        }
    }
}

impl ServerProperties {
    pub fn load_or_create<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let path = path.as_ref();

        if !path.exists() {
            let config = Self::default();
            config.save(path)?;
            return Ok(config);
        }

        let raw = fs::read_to_string(path)?;
        Ok(serde_java_properties::from_str(&raw)?)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), ConfigError> {
        let content = serde_java_properties::to_string(&self)?.replace("\\", "");
        fs::write(path, content)?;
        Ok(())
    }
}
