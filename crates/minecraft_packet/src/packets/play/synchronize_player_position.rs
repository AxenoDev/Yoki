use crate::packet::{OutgoingPacket, PacketDirection, PacketMeta};
use yoki_binutils::{
    BinaryWriter, ProtocolError, WriteBytes,
    data_types::VarInt,
};

#[derive(Debug, Clone)]
pub struct SynchronizePlayerPositionPacket {
    teleport_id: VarInt,
    x: f64,
    y: f64,
    z: f64,
    velocity_x: f64,
    velocity_y: f64,
    velocity_z: f64,
    yaw: f32,
    pitch: f32,
    flags: i32,
}

impl SynchronizePlayerPositionPacket {
    pub fn spawn(x: f64, y: f64, z: f64, yaw: f32, pitch: f32) -> Self {
        Self {
            teleport_id: VarInt::new(0),
            x,
            y,
            z,
            velocity_x: 0.0,
            velocity_y: 0.0,
            velocity_z: 0.0,
            yaw,
            pitch,
            flags: 0,
        }
    }
}

impl PacketMeta for SynchronizePlayerPositionPacket {
    const DIRECTION: PacketDirection = PacketDirection::Out;
}

impl OutgoingPacket for SynchronizePlayerPositionPacket {
    fn encode_payload(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        self.teleport_id.write(writer)?;
        self.x.write(writer)?;
        self.y.write(writer)?;
        self.z.write(writer)?;
        self.velocity_x.write(writer)?;
        self.velocity_y.write(writer)?;
        self.velocity_z.write(writer)?;
        self.yaw.write(writer)?;
        self.pitch.write(writer)?;
        self.flags.write(writer)?;
        Ok(())
    }
}
