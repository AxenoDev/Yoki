use std::path::Path;

use crate::configuration::{ConfigError, Configuration};
use crate::registry::RegistryStore;
use crate::server::players::PlayerRegistry;

#[derive(Debug)]
pub struct ServerState {
    pub config: Configuration,
    registry_store: RegistryStore,
    players: PlayerRegistry,
    allow_unsupported_versions: bool,
    reply_to_status: bool,
    accept_transfers: bool,
}

impl ServerState {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let config = Configuration::load(path)?;
        let registry_store =
            RegistryStore::load().map_err(|err| ConfigError::Registry(err.to_string()))?;
        Ok(Self::new(config, registry_store))
    }

    pub fn new(config: Configuration, registry_store: RegistryStore) -> Self {
        Self {
            allow_unsupported_versions: false,
            reply_to_status: true,
            accept_transfers: false,
            config,
            registry_store,
            players: PlayerRegistry::default(),
        }
    }

    pub fn registry_store(&self) -> &RegistryStore {
        &self.registry_store
    }

    pub fn players(&self) -> &PlayerRegistry {
        &self.players
    }

    pub fn config(&self) -> &Configuration {
        &self.config
    }

    pub fn bind(&self) -> String {
        format!("{}:{}", self.config.server.addr, self.config.server.port)
    }

    pub fn max_players(&self) -> usize {
        self.config.server.max_players
    }

    pub fn motd(&self) -> &str {
        &self.config.motd.motd
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
}

impl Default for ServerState {
    fn default() -> Self {
        Self::new(
            Configuration::default(),
            RegistryStore::load().expect("failed to load registry data"),
        )
    }
}
