use takumi_binutils::{ProtocolError, reader::PacketReader};

use crate::packet::{IncomingPacket, PacketDirection, PacketMeta};

#[derive(Debug, Clone)]
pub struct LoginPluginResponsePacket {
    pub message_id: i32,
    pub is_present: bool,
    pub data: Vec<u8>,
}

impl PacketMeta for LoginPluginResponsePacket {
    const ID: i32 = 0x02;
    const DIRECTION: PacketDirection = PacketDirection::In;
}

impl IncomingPacket for LoginPluginResponsePacket {
    fn decode_payload(reader: &mut PacketReader<'_>) -> Result<Self, ProtocolError> {
        let message_id = reader.read_varint()?;
        let is_present = reader.read_bool()?;
        let data = if is_present {
            reader.read_remaining_bytes()
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
