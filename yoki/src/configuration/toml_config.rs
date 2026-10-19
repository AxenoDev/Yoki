use serde::{Serialize, de::DeserializeOwned};
use std::{fs, path::Path};

use crate::configuration::ConfigError;

pub trait TomlConfig: Serialize + DeserializeOwned + Default {
    fn file_name() -> &'static str;

    fn header() -> String;

    fn on_load(&mut self) -> Result<(), ConfigError> {
        Ok(())
    }

    fn load_or_create(dir: &Path) -> Result<Self, ConfigError> {
        let path = dir.join(Self::file_name());
        let mut config = if !path.exists() {
            let config = Self::default();
            config.save(dir)?;
            config
        } else {
            let content = fs::read_to_string(path)?;
            toml::from_str(&content)?
        };

        config.on_load()?;
        Ok(config)
    }

    fn save(&self, dir: &Path) -> Result<(), ConfigError> {
        fs::create_dir_all(dir)?;
        let content = format!("{}{}\n", Self::header(), toml::to_string_pretty(self)?);
        fs::write(dir.join(Self::file_name()), content)?;
        Ok(())
    }
}
