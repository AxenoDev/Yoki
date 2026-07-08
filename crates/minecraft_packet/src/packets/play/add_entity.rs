use crate::packet::{OutgoingPacket, PacketDirection, PacketMeta};
use crate::packets::play::data::angle_from_degrees;
use uuid::Uuid;
use yoki_binutils::{BinaryWriter, ProtocolError, WriteBytes, data_types::VarInt};

#[derive(Debug, Clone)]
pub struct AddEntityPacket {
    entity_id: VarInt,
    entity_uuid: Uuid,
    entity_type: VarInt,
    x: f64,
    y: f64,
    z: f64,
    pitch: u8,
    yaw: u8,
    head_yaw: u8,
    data: VarInt,
}

impl AddEntityPacket {
    #[allow(clippy::too_many_arguments)]
    pub fn player(
        entity_id: i32,
        entity_uuid: Uuid,
        player_entity_type: i32,
        x: f64,
        y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
    ) -> Self {
        Self {
            entity_id: VarInt::new(entity_id),
            entity_uuid,
            entity_type: VarInt::new(player_entity_type),
            x,
            y,
            z,
            pitch: angle_from_degrees(pitch),
            yaw: angle_from_degrees(yaw),
            head_yaw: angle_from_degrees(yaw),
            data: VarInt::new(0),
        }
    }
}

impl PacketMeta for AddEntityPacket {
    const DIRECTION: PacketDirection = PacketDirection::Out;
}

impl OutgoingPacket for AddEntityPacket {
    fn encode_payload(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        self.entity_id.write(writer)?;
        self.entity_uuid.write(writer)?;
        self.entity_type.write(writer)?;
        self.x.write(writer)?;
        self.y.write(writer)?;
        self.z.write(writer)?;
        // Velocity as LpVec3: a zero vector encodes as the single byte 0x00.
        writer.write_byte(0x00);
        writer.write_byte(self.pitch);
        writer.write_byte(self.yaw);
        writer.write_byte(self.head_yaw);
        self.data.write(writer)?;
        Ok(())
    }
}
