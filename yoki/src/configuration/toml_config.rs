use serde::{Serialize, de::DeserializeOwned};
use std::{fs, path::Path};

use crate::configuration::ConfigError;

pub trait TomlConfig: Serialize + DeserializeOwned + Default {
    const FILE_NAME: &'static str;
    const HEADER: &'static str = "";

    fn load_or_create(dir: &Path) -> Result<Self, ConfigError> {
        let path = dir.join(Self::FILE_NAME);
        if !path.exists() {
            let config = Self::default();
            config.save(dir)?;
            return Ok(config);
        }
        Ok(toml::from_str(&fs::read_to_string(path)?)?)
    }

    fn save(&self, dir: &Path) -> Result<(), ConfigError> {
        fs::create_dir_all(dir)?;
        let content = format!("{}{}\n", Self::HEADER, toml::to_string_pretty(self)?);
        fs::write(dir.join(Self::FILE_NAME), content)?;
        Ok(())
    }
}
