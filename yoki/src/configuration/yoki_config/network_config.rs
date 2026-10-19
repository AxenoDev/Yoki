use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct NetworkConfig {
    pub reply_to_status: bool,
    pub accept_transfers: bool,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            reply_to_status: true,
            accept_transfers: false,
        }
    }
}
