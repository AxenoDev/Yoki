use base64::engine::general_purpose;
use base64::{Engine, alphabet, engine};
use protocol_version::protocol_version::ProtocolVersion;

use crate::commands::{CommandRegistry, impls::register_all};
use crate::configuration::ConfigError;
use crate::configuration::server_properties::ServerProperties;
use crate::configuration::toml_config::TomlConfig;
use crate::configuration::yoki_config::YokiConfig;
use crate::registry::RegistryStore;
use crate::server::players::PlayerRegistry;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
pub struct ServerState {
    pub server_properties: ServerProperties,
    pub yoki_configuration: YokiConfig,
    registry_store: RegistryStore,
    players: PlayerRegistry,
    commands: CommandRegistry,
    favicon: Option<String>,
}

impl ServerState {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let server_properties = ServerProperties::load_or_create(path.as_ref())?;
        let yoki_configuration = YokiConfig::load_or_create(path.as_ref())?;
        let registry_store =
            RegistryStore::load().map_err(|err| ConfigError::Registry(err.to_string()))?;
        let favicon = match Self::load_favicon(path.as_ref().join("server-icon.png")) {
            Ok(favicon) => Some(favicon),
            Err(err) => {
                tracing::warn!("Failed to load favicon: {}", err);
                Self::default().favicon
            }
        };
        Ok(Self::new(
            server_properties,
            yoki_configuration,
            registry_store,
            favicon
        ))
    }

    pub fn new(
        server_properties: ServerProperties,
        yoki_configuration: YokiConfig,
        registry_store: RegistryStore,
        favicon: Option<String>,
    ) -> Self {
        Self {
            server_properties,
            yoki_configuration,
            registry_store,
            players: PlayerRegistry::default(),
            commands: register_all(),
            favicon,
        }
    }

    fn load_favicon<P>(file_path: P) -> Result<String, ConfigError>
    where
        P: AsRef<Path>,
    {
        let mut file = File::open(file_path)?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let engine = engine::GeneralPurpose::new(&alphabet::STANDARD, general_purpose::PAD);
        let base64_encoded = engine.encode(&buffer);

        tracing::debug!("Loaded favicon, base64 length: {}", base64_encoded.len());
        Ok(format!("data:image/png;base64,{base64_encoded}"))
    }

    pub fn favicon(&self) -> Option<&str> {
        self.favicon.as_deref()
    }

    pub fn registry_store(&self) -> &RegistryStore {
        &self.registry_store
    }

    pub fn commands(&self) -> &CommandRegistry {
        &self.commands
    }

    pub fn players(&self) -> &PlayerRegistry {
        &self.players
    }

    pub fn server_properties(&self) -> &ServerProperties {
        &self.server_properties
    }

    pub fn yoki_configuration(&self) -> &YokiConfig {
        &self.yoki_configuration
    }

    pub fn bind(&self) -> String {
        format!(
            "{}:{}",
            self.server_properties.server_ip, self.server_properties.server_port
        )
    }

    pub fn is_version_allowed(&self, version: ProtocolVersion) -> bool {
        self.yoki_configuration
            .versioning
            .is_version_allowed(version)
    }

    pub fn version_kick_message(&self, client_version: &str) -> String {
        self.yoki_configuration
            .versioning
            .kick_message_for(client_version)
    }

    pub const fn allow_unsupported_versions(&self) -> bool {
        self.yoki_configuration
            .versioning
            .allow_unsupported_versions
    }

    pub const fn reply_to_status(&self) -> bool {
        self.yoki_configuration.network.reply_to_status
    }

    pub const fn accept_transfers(&self) -> bool {
        self.yoki_configuration.network.accept_transfers
    }

    pub fn broadcast(&self, message: impl Into<String>) {
        let line = message.into();
        for player in self.players.all() {
            player.send(line.clone());
        }
    }
}

impl Default for ServerState {
    fn default() -> Self {
        let favicon_text = "data:image/png;base64,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".to_string();
        Self::new(
            ServerProperties::default(),
            YokiConfig::default(),
            RegistryStore::load().expect("failed to load registry data"),
            Some(favicon_text)
        )
    }
}
