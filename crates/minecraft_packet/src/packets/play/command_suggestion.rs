use crate::packet::{IncomingPacket, PacketDirection, PacketMeta};
use yoki_binutils::{data_types::VarInt, BinaryReader, ProtocolError, ReadBytes};

#[derive(Debug, Clone)]
pub struct CommandSuggestionPacket {
    pub id: i32,
    pub text: String,
}

impl PacketMeta for CommandSuggestionPacket {
    const DIRECTION: PacketDirection = PacketDirection::In;
}

impl IncomingPacket for CommandSuggestionPacket {
    fn decode_payload(reader: &mut BinaryReader<'_>) -> Result<Self, ProtocolError> {
        Ok(Self {
            id: VarInt::read(reader)?.inner(),
            text: String::read(reader)?,
        })
    }
}
