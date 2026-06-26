use std::fmt;

use minecraft_protocol::State;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProtocolError {
    UnexpectedEof,
    VarIntTooBig,
    InvalidUtf8,
    InvalidUuid,
    InvalidIntent(i32),
    UnknownPacket { id: i32, conn: Option<State> },
    Io(String),
}

impl fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedEof => f.write_str("unexpected end of packet"),
            Self::VarIntTooBig => f.write_str("varint is too big"),
            Self::InvalidUtf8 => f.write_str("invalid UTF-8 string"),
            Self::InvalidUuid => f.write_str("invalid UUID"),
            Self::InvalidIntent(v) => f.write_str(&format!("unknown handshake intent: {v}")),
            Self::UnknownPacket { id, conn } => f.write_str(&format!(
                "unknown packet id: 0x{id:02X}, connection state: {conn:?}"
            )),
            Self::Io(msg) => f.write_str(&format!("io error: {msg}")),
        }
    }
}

impl std::error::Error for ProtocolError {}

impl From<std::io::Error> for ProtocolError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err.to_string())
    }
}
