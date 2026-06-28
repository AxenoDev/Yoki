use crate::Config;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Use `0.0.0.0` to bind on all interfaces.
    pub addr: String,

    /// The port to bind the server to.
    pub port: u16,

    /// Maximum number of players allowed on the server.
    pub max_players: usize,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            addr: "0.0.0.0".to_string(),
            port: 25565,
            max_players: 20,
        }
    }
}

impl Config for ServerConfig {
    const FILE_NAME: &'static str = "server.toml";
}
