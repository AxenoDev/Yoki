use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::configuration::{ConfigError, Configuration};

#[derive(Debug)]
pub struct ServerState {
    pub config: Configuration,
    online_players: AtomicUsize,
    allow_unsupported_versions: bool,
    reply_to_status: bool,
    accept_transfers: bool,
}

impl ServerState {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let config = Configuration::load(path)?;
        Ok(Self::new(config))
    }

    pub fn new(config: Configuration) -> Self {
        Self {
            online_players: AtomicUsize::new(0),
            allow_unsupported_versions: false,
            reply_to_status: true,
            accept_transfers: false,
            config,
        }
    }

    pub fn config(&self) -> &Configuration {
        &self.config
    }

    pub fn bind(&self) -> String {
        format!("{}:{}", self.config.server.addr, self.config.server.port)
    }

    pub fn online_players(&self) -> usize {
        self.online_players.load(Ordering::Relaxed)
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

    pub fn increment(&self) {
        self.online_players.fetch_add(1, Ordering::Relaxed);
    }

    pub fn decrement(&self) {
        self.online_players.fetch_sub(1, Ordering::Relaxed);
    }
}

impl Default for ServerState {
    fn default() -> Self {
        Self::new(Configuration::default())
    }
}
