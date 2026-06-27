use thiserror::Error;

use crate::ServerState;

use super::batch::Batch;
use super::client_state::ClientState;

#[derive(Error, Debug)]
pub enum PacketHandlerError {
    #[error("An error occurred while handling a packet: {0}")]
    Custom(String),
    #[error("{0}")]
    InvalidState(String, bool),
}

impl PacketHandlerError {
    pub fn custom(message: impl Into<String>) -> Self {
        Self::Custom(message.into())
    }

    pub fn invalid_state(message: impl Into<String>) -> Self {
        Self::InvalidState(message.into(), false)
    }

    pub fn disconnect(message: impl Into<String>) -> Self {
        Self::InvalidState(message.into(), true)
    }
}

pub trait PacketHandler {
    fn handle(
        &self,
        client_state: &mut ClientState,
        server_state: &ServerState,
    ) -> Result<Batch, PacketHandlerError>;
}
