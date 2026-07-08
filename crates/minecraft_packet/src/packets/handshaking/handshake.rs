use minecraft_protocol::State;
use yoki_binutils::{BinaryReader, ProtocolError, ProtocolRead, data_types::VarInt};
use yoki_macros::PacketIn;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Intent {
    Status,
    Login,
    Transfer,
}

impl Intent {
    pub fn from_varint(value: i32) -> Result<Self, ProtocolError> {
        match value {
            1 => Ok(Self::Status),
            2 => Ok(Self::Login),
            3 => Ok(Self::Transfer),
            _ => Err(ProtocolError::InvalidIntent(value)),
        }
    }

    pub const fn as_i32(self) -> i32 {
        match self {
            Self::Status => 1,
            Self::Login => 2,
            Self::Transfer => 3,
        }
    }
}

impl ProtocolRead for Intent {
    fn read_from(reader: &mut BinaryReader<'_>) -> Result<Self, ProtocolError> {
        Self::from_varint(VarInt::read_from(reader)?.inner())
    }
}

impl From<Intent> for State {
    fn from(intent: Intent) -> Self {
        match intent {
            Intent::Status => State::Status,
            Intent::Login => State::Login,
            Intent::Transfer => State::Transfer,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PacketIn)]
pub struct HandshakePacket {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    /// 1: Status 2: Login 3: Transfer
    pub intent: VarInt,
}

impl HandshakePacket {
    pub fn status(
        protocol_version: i32,
        server_address: impl Into<String>,
        server_port: u16,
    ) -> Self {
        Self {
            protocol_version: protocol_version.into(),
            server_address: server_address.into(),
            server_port,
            intent: Intent::Status.as_i32().into(),
        }
    }

    pub fn login(
        protocol_version: i32,
        server_address: impl Into<String>,
        server_port: u16,
    ) -> Self {
        Self {
            protocol_version: protocol_version.into(),
            server_address: server_address.into(),
            server_port,
            intent: Intent::Login.as_i32().into(),
        }
    }
}
