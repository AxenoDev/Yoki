use crate::packet::{OutgoingPacket, PacketDirection, PacketMeta};
use crate::packets::play::data::angle_from_degrees;
use yoki_binutils::{BinaryWriter, ProtocolError, WriteBytes, data_types::VarInt};

#[derive(Debug, Clone)]
pub struct RotateHeadPacket {
    entity_id: VarInt,
    head_yaw: u8,
}

impl RotateHeadPacket {
    pub fn new(entity_id: i32, yaw_degrees: f32) -> Self {
        Self {
            entity_id: VarInt::new(entity_id),
            head_yaw: angle_from_degrees(yaw_degrees),
        }
    }
}

impl PacketMeta for RotateHeadPacket {
    const DIRECTION: PacketDirection = PacketDirection::Out;
}

impl OutgoingPacket for RotateHeadPacket {
    fn encode_payload(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        self.entity_id.write(writer)?;
        writer.write_byte(self.head_yaw);
        Ok(())
    }
}
