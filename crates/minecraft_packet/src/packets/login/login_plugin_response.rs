use yoki_binutils::{BinaryReader, ProtocolError, ProtocolRead, data_types::VarInt};

use crate::packet::{IncomingPacket, PacketDirection, PacketMeta};

#[derive(Debug, Clone)]
pub struct LoginPluginResponsePacket {
    pub message_id: i32,
    pub is_present: bool,
    pub data: Vec<u8>,
}

impl PacketMeta for LoginPluginResponsePacket {
    const DIRECTION: PacketDirection = PacketDirection::In;
}

impl IncomingPacket for LoginPluginResponsePacket {
    fn decode_payload(reader: &mut BinaryReader<'_>) -> Result<Self, ProtocolError> {
        let message_id = VarInt::read_from(reader)?.inner();
        let is_present = bool::read_from(reader)?;
        let data = if is_present {
            reader.take_remaining_bytes()
        } else {
            Vec::new()
        };

        Ok(Self {
            message_id,
            is_present,
            data,
        })
    }
}
