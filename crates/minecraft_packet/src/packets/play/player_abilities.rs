use crate::packet::{OutgoingPacket, PacketDirection, PacketMeta};
use yoki_binutils::{BinaryWriter, ProtocolError, WriteBytes};

#[derive(Debug, Clone)]
pub struct PlayerAbilitiesPacket {
    flags: i8,
    flying_speed: f32,
    field_of_view_modifier: f32,
}

impl Default for PlayerAbilitiesPacket {
    fn default() -> Self {
        Self {
            flags: 0,
            flying_speed: 0.05,
            field_of_view_modifier: 0.1,
        }
    }
}

impl PacketMeta for PlayerAbilitiesPacket {
    const DIRECTION: PacketDirection = PacketDirection::Out;
}

impl OutgoingPacket for PlayerAbilitiesPacket {
    fn encode_payload(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        self.flags.write(writer)?;
        self.flying_speed.write(writer)?;
        self.field_of_view_modifier.write(writer)?;
        Ok(())
    }
}
