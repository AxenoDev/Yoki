use protocol_version::protocol_version::ProtocolVersion;
use serde::{Deserialize, Serialize};

use crate::configuration::{
    YOKI_CONFIG_FILE_NAME,
    toml_config::TomlConfig,
    version_spec::{VersionSpec, known_versions},
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct YokiConfig {
    pub versioning: VersioningConfig,
    pub development: DevelopmentConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct VersioningConfig {
    pub allowed: Vec<VersionSpec>,
}

impl Default for VersioningConfig {
    fn default() -> Self {
        let oldest = known_versions()
            .into_iter()
            .next()
            .expect("at least one known protocol version");
        Self {
            allowed: vec![VersionSpec::at_least(oldest)],
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct DevelopmentConfig {
    pub debug: bool,
}

impl TomlConfig for YokiConfig {
    const FILE_NAME: &'static str = YOKI_CONFIG_FILE_NAME;
    const HEADER: &'static str = "#Yoki configuration\n";
}

impl YokiConfig {
    pub fn is_version_allowed(&self, version: ProtocolVersion) -> bool {
        version.protocol_number() <= 0 || self.versioning.allowed.iter().any(|s| s.matches(version))
    }
}