use protocol_version::protocol_version::ProtocolVersion;
use serde::{Deserialize, Serialize};
use yoki_macros::PacketOut;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerStatus {
    pub version: VersionInfo,
    pub players: PlayersInfo,
    pub description: TextComponent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforces_secure_chat: Option<bool>,
}

impl ServerStatus {
    pub fn yoki_default() -> Self {
        Self {
            version: VersionInfo::from(ProtocolVersion::latest()),
            players: PlayersInfo::default(),
            description: TextComponent::new("Yoki Server"),
            favicon: None,
            enforces_secure_chat: None,
        }
    }

    pub fn with_version(mut self, version: VersionInfo) -> Self {
        self.version = version;
        self
    }

    pub fn with_players(mut self, online: u32, max: u32) -> Self {
        self.players.online = online;
        self.players.max = max;
        self
    }

    pub fn with_player_sample(mut self, sample: Vec<PlayerSample>) -> Self {
        self.players.sample = Some(sample);
        self
    }

    pub fn with_description(mut self, description: impl Into<TextComponent>) -> Self {
        self.description = description.into();
        self
    }

    pub fn with_favicon(mut self, favicon: impl Into<String>) -> Self {
        self.favicon = Some(favicon.into());
        self
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionInfo {
    pub name: String,
    pub protocol: i32,
}

impl VersionInfo {
    pub fn new(name: impl Into<String>, protocol: i32) -> Self {
        Self {
            name: name.into(),
            protocol,
        }
    }
}

impl From<ProtocolVersion> for VersionInfo {
    fn from(version: ProtocolVersion) -> Self {
        Self {
            name: version
                .known_packs()
                .first()
                .copied()
                .unwrap_or("unknown")
                .to_string(),
            protocol: version.protocol_number(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayersInfo {
    pub max: u32,
    pub online: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample: Option<Vec<PlayerSample>>,
}

impl Default for PlayersInfo {
    fn default() -> Self {
        Self {
            max: 100,
            online: 0,
            sample: None,
        }
    }
}

impl PlayersInfo {
    pub fn new(online: u32, max: u32) -> Self {
        Self {
            online,
            max,
            sample: None,
        }
    }

    pub fn with_sample(mut self, sample: Vec<PlayerSample>) -> Self {
        self.sample = Some(sample);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerSample {
    pub name: String,
    pub id: String,
}

impl PlayerSample {
    pub fn new(name: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            id: id.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextComponent {
    Text { text: String },
}

impl TextComponent {
    pub fn new(text: impl Into<String>) -> Self {
        Self::Text { text: text.into() }
    }
}

impl From<&str> for TextComponent {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for TextComponent {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PacketOut)]
#[packet(id = 0x00)]
pub struct StatusResponsePacket {
    pub json: String,
}

impl StatusResponsePacket {
    pub fn yoki_default() -> Self {
        Self::from_status(&ServerStatus::yoki_default())
    }

    pub fn from_status(status: &ServerStatus) -> Self {
        Self {
            json: status
                .to_json()
                .expect("ServerStatus serialization is infallible"),
        }
    }
}
