use crate::packet::{IncomingPacket, PacketDirection, PacketMeta};
use yoki_binutils::{BinaryReader, ProtocolError, ReadBytes};

#[derive(Debug, Clone, Copy)]
pub struct SetCarriedItemPacket {
    pub slot: i16,
}

impl PacketMeta for SetCarriedItemPacket {
    const DIRECTION: PacketDirection = PacketDirection::In;
}

impl IncomingPacket for SetCarriedItemPacket {
    fn decode_payload(reader: &mut BinaryReader<'_>) -> Result<Self, ProtocolError> {
        Ok(Self {
            slot: u16::read(reader)? as i16,
        })
    }
}
