use serde::{Deserialize, Serialize};

use crate::configuration::{
    ConfigError,
    toml_config::TomlConfig,
    yoki_config::{
        development_config::DevelopmentConfig,
        network_config::NetworkConfig,
        versioning_config::{VersioningConfig, version_spec::known_versions},
    },
};

pub mod development_config;
pub mod network_config;
pub mod versioning_config;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct YokiConfig {
    pub versioning: VersioningConfig,
    pub network: NetworkConfig,
    pub development: DevelopmentConfig,
}

impl TomlConfig for YokiConfig {
    fn file_name() -> &'static str {
        "yoki.toml"
    }

    fn header() -> String {
        let known = known_versions()
            .iter()
            .map(|version| version.humanize())
            .collect::<Vec<_>>()
            .join(", ");

        format!(
            "# Yoki configuration\n\
             #\n\
             # allow-version: client versions allowed to join. Each entry is either:\n\
             #   a single version      \"26.2\"\n\
             #   a range (inclusive)   \"[26.1,26.3]\"\n\
             #   a range (exclusive)   \"[26.1,26.3)\"  26.1 up to, but not including, 26.3\n\
             #   everything from       \"[26.1,)\"       26.1 and newer\n\
             #   everything up to      \"(,26.2]\"       26.2 and older\n\
             # Known versions: {known}\n\
             #\n\
             # kick-message: shown to clients whose version is not allowed.\n\
             #   {{version}} = the client's version, {{allowed}} = the allowed versions\n"
        )
    }

    fn on_load(&mut self) -> Result<(), ConfigError> {
        if self.versioning.allowed.is_empty() {
            return Err(ConfigError::NoAllowedVersion);
        }
        Ok(())
    }
}
