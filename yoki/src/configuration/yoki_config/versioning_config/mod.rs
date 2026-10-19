use protocol_version::protocol_version::ProtocolVersion;
use serde::{Deserialize, Serialize};

use crate::configuration::yoki_config::versioning_config::version_spec::{
    VersionSpec, known_versions,
};

pub mod version_spec;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct VersioningConfig {
    pub allowed: Vec<VersionSpec>,
    pub allow_unsupported_versions: bool,
    pub kick_message: String,
}

impl Default for VersioningConfig {
    fn default() -> Self {
        let oldest = known_versions()
            .into_iter()
            .next()
            .expect("at least one known protocol version");
        Self {
            allowed: vec![VersionSpec::at_least(oldest)],
            allow_unsupported_versions: false,
            kick_message: "Your version ({version}) is not suppoorted by this server. Supported versions: {allowed}".to_string(),
        }
    }
}

impl VersioningConfig {
    pub fn is_version_allowed(&self, version: ProtocolVersion) -> bool {
        version.protocol_number() <= 0 || self.allowed.iter().any(|s| s.matches(version))
    }

    pub fn kick_message_for(&self, client_version: &str) -> String {
        let allowed = known_versions()
            .into_iter()
            .filter(|v| self.is_version_allowed(*v))
            .map(|v| v.humanize())
            .collect::<Vec<_>>()
            .join(", ");

        self.kick_message
            .replace("{version}", client_version)
            .replace("{allowed}", &allowed)
    }
}
