use serde::{Deserialize, Serialize};
use std::{
    fs::{self},
    path::Path,
};

use crate::configuration::ConfigError;

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
    pub fn load_or_create(dir: &Path) -> Result<Self, ConfigError> {
        let path = dir.join("server.properties");

        if !path.exists() {
            let config = Self::default();
            config.save(dir)?;
            return Ok(config);
        }

        let raw = fs::read_to_string(path)?;
        Ok(serde_java_properties::from_str(&raw)?)
    }

    pub fn save(&self, path: &Path) -> Result<(), ConfigError> {
        let content = serde_java_properties::to_string(&self)?.replace("\\", "");
        fs::write(path.join("server.properties"), content)?;
        Ok(())
    }
}
