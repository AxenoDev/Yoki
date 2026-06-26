use minecraft_protocol::{Direction, State};
use protocol_version::protocol_version::ProtocolVersion;
use uuid::Uuid;

use crate::server::game_profile::GameProfile;

pub struct ClientState {
    clientbound_state: State,
    serverbound_state: State,
    protocol_version: ProtocolVersion,
    kick_message: Option<String>,
    game_profile: Option<GameProfile>,
}

impl Default for ClientState {
    fn default() -> Self {
        Self {
            clientbound_state: State::Handshaking,
            serverbound_state: State::Handshaking,
            protocol_version: ProtocolVersion::Any,
            kick_message: None,
            game_profile: None,
        }
    }
}

impl ClientState {
    pub fn new() -> Self {
        Self {
            clientbound_state: State::Handshaking,
            serverbound_state: State::Handshaking,
            ..Default::default()
        }
    }

    pub fn kick(&mut self, message: impl Into<String>) {
        self.kick_message = Some(message.into());
    }

    pub fn should_kick(&self) -> Option<&str> {
        self.kick_message.as_deref()
    }

    pub const fn clientbound_state(&self) -> State {
        self.clientbound_state
    }

    pub const fn serverbound_state(&self) -> State {
        self.serverbound_state
    }

    pub const fn set_state(&mut self, direction: Direction, new_state: State) {
        match direction {
            Direction::Clientbound => self.clientbound_state = new_state,
            Direction::Serverbound => self.serverbound_state = new_state,
        }
    }

    pub const fn protocol_version(&self) -> ProtocolVersion {
        self.protocol_version
    }

    pub const fn set_protocol_version(&mut self, new_protocol_version: ProtocolVersion) {
        self.protocol_version = new_protocol_version;
    }

    pub fn set_game_profile(&mut self, game_profile: GameProfile) {
        if let Some(ref mut existing_game_profile) = self.game_profile {
            existing_game_profile.set_name(game_profile.username());
        } else {
            self.game_profile = Some(game_profile);
        }
    }

    pub fn game_profile(&self) -> Option<GameProfile> {
        self.game_profile.clone()
    }

    pub fn get_username(&self) -> String {
        self.game_profile()
            .map_or_else(String::new, |profile| profile.username().to_string())
    }

    pub fn get_unique_id(&self) -> Uuid {
        self.game_profile()
            .map_or_else(Uuid::default, |profile| profile.uuid())
    }
}
