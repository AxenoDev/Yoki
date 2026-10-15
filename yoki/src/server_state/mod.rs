use crate::ConfigError;
use crate::commands::{CommandRegistry, impls::register_all};
use crate::configuration::PROPERTIES_FILE_NAME;
use crate::configuration::ServerProperties;
use crate::registry::RegistryStore;
use crate::server::players::PlayerRegistry;
use std::path::Path;

#[derive(Debug)]
pub struct ServerState {
    pub server_properties: ServerProperties,
    registry_store: RegistryStore,
    players: PlayerRegistry,
    commands: CommandRegistry,
    allow_unsupported_versions: bool,
    reply_to_status: bool,
    accept_transfers: bool,
}

impl ServerState {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let config = ServerProperties::load_or_create(path.as_ref().join(PROPERTIES_FILE_NAME))?;
        let registry_store =
            RegistryStore::load().map_err(|err| ConfigError::Registry(err.to_string()))?;
        Ok(Self::new(config, registry_store))
    }

    pub fn new(server_properties: ServerProperties, registry_store: RegistryStore) -> Self {
        Self {
            allow_unsupported_versions: false,
            reply_to_status: true,
            accept_transfers: false,
            server_properties,
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
            RegistryStore::load().expect("failed to load registry data"),
        )
    }
}
