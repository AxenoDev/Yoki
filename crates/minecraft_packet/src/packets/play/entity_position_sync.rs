use crate::packet::{OutgoingPacket, PacketDirection, PacketMeta};
use yoki_binutils::{BinaryWriter, ProtocolError, WriteBytes, data_types::VarInt};

#[derive(Debug, Clone)]
pub struct EntityPositionSyncPacket {
    entity_id: VarInt,
    x: f64,
    y: f64,
    z: f64,
    yaw: f32,
    pitch: f32,
    on_ground: bool,
}

impl EntityPositionSyncPacket {
    pub fn new(
        entity_id: i32,
        x: f64,
        y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
        on_ground: bool,
    ) -> Self {
        Self {
            entity_id: VarInt::new(entity_id),
            x,
            y,
            z,
            yaw,
            pitch,
            on_ground,
        }
    }
}

impl PacketMeta for EntityPositionSyncPacket {
    const DIRECTION: PacketDirection = PacketDirection::Out;
}

impl OutgoingPacket for EntityPositionSyncPacket {
    fn encode_payload(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        self.entity_id.write(writer)?;
        self.x.write(writer)?;
        self.y.write(writer)?;
        self.z.write(writer)?;
        0f64.write(writer)?; // velocity x
        0f64.write(writer)?; // velocity y
        0f64.write(writer)?; // velocity z
        self.yaw.write(writer)?;
        self.pitch.write(writer)?;
        self.on_ground.write(writer)?;
        Ok(())
    }
}
