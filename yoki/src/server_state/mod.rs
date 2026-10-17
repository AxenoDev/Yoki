use protocol_version::protocol_version::ProtocolVersion;

use crate::commands::{CommandRegistry, impls::register_all};
use crate::configuration::ConfigError;
use crate::configuration::server_properties::{ServerProperties};
use crate::configuration::toml_config::TomlConfig;
use crate::configuration::yoki_config::YokiConfig;
use crate::registry::RegistryStore;
use crate::server::players::PlayerRegistry;
use std::path::Path;

#[derive(Debug)]
pub struct ServerState {
    pub server_properties: ServerProperties,
    pub yoki_configuration: YokiConfig,
    registry_store: RegistryStore,
    players: PlayerRegistry,
    commands: CommandRegistry,
    allow_unsupported_versions: bool,
    reply_to_status: bool,
    accept_transfers: bool,
}

impl ServerState {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let server_properties = ServerProperties::load_or_create(path.as_ref())?;
        let yoki_configuration = YokiConfig::load_or_create(path.as_ref())?;
        let registry_store =
            RegistryStore::load().map_err(|err| ConfigError::Registry(err.to_string()))?;
        Ok(Self::new(
            server_properties,
            yoki_configuration,
            registry_store,
        ))
    }

    pub fn new(
        server_properties: ServerProperties,
        yoki_configuration: YokiConfig,
        registry_store: RegistryStore,
    ) -> Self {
        Self {
            allow_unsupported_versions: false,
            reply_to_status: true,
            accept_transfers: false,
            server_properties,
            yoki_configuration,
            registry_store,
            players: PlayerRegistry::default(),
            commands: register_all(),
        }
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

    pub fn max_players(&self) -> usize {
        self.server_properties.max_players
    }

    pub fn motd(&self) -> &str {
        &self.server_properties.motd
    }

    pub fn is_version_allowed(&self, version: ProtocolVersion) -> bool {
        self.yoki_configuration.is_version_allowed(version)
    }

    pub const fn allow_unsupported_versions(&self) -> bool {
        self.allow_unsupported_versions
    }

    pub const fn reply_to_status(&self) -> bool {
        self.reply_to_status
    }

    pub const fn accept_transfers(&self) -> bool {
        self.accept_transfers
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
        Self::new(
            ServerProperties::default(),
            YokiConfig::default(),
            RegistryStore::load().expect("failed to load registry data"),
        )
    }
}
