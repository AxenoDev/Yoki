use crate::packet::{OutgoingPacket, PacketDirection, PacketMeta};
use yoki_binutils::{BinaryWriter, ProtocolError, WriteBytes, data_types::VarInt};

pub const ANIMATION_SWING_MAIN_ARM: u8 = 0;
pub const ANIMATION_SWING_OFFHAND: u8 = 3;

#[derive(Debug, Clone)]
pub struct AnimatePacket {
    entity_id: VarInt,
    animation: u8,
}

impl AnimatePacket {
    pub fn new(entity_id: i32, animation: u8) -> Self {
        Self {
            entity_id: VarInt::new(entity_id),
            animation,
        }
    }
}

impl PacketMeta for AnimatePacket {
    const DIRECTION: PacketDirection = PacketDirection::Out;
}

impl OutgoingPacket for AnimatePacket {
    fn encode_payload(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        self.entity_id.write(writer)?;
        writer.write_byte(self.animation);
        Ok(())
    }
}
