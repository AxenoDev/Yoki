use crate::packet::{IncomingPacket, PacketDirection, PacketMeta};
use yoki_binutils::{BinaryReader, ProtocolError, ReadBytes, data_types::VarInt};

pub const HAND_MAIN: i32 = 0;
pub const HAND_OFF: i32 = 1;

#[derive(Debug, Clone, Copy)]
pub struct SwingPacket {
    pub hand: i32,
}

impl PacketMeta for SwingPacket {
    const DIRECTION: PacketDirection = PacketDirection::In;
}

impl IncomingPacket for SwingPacket {
    fn decode_payload(reader: &mut BinaryReader<'_>) -> Result<Self, ProtocolError> {
        Ok(Self {
            hand: VarInt::read(reader)?.inner(),
        })
    }
}
