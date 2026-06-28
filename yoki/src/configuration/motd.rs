use crate::Config;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotdConfig {
    /// The message of the day (MOTD) displayed in the server list.
    pub motd: String,
}

impl Default for MotdConfig {
    fn default() -> Self {
        Self {
            motd: "Welcome to Yoki server!".to_string(),
        }
    }
}

impl Config for MotdConfig {
    const FILE_NAME: &'static str = "motd.toml";
}
