use crate::packet::{OutgoingPacket, PacketDirection, PacketMeta};
use yoki_binutils::{BinaryWriter, ProtocolError, WriteBytes};

#[derive(Debug, Clone)]
pub struct GameEventPacket {
    event: u8,
    value: f32,
}

impl GameEventPacket {
    pub fn start_waiting_for_chunks() -> Self {
        Self {
            event: 13,
            value: 0.0,
        }
    }
}

impl PacketMeta for GameEventPacket {
    const DIRECTION: PacketDirection = PacketDirection::Out;
}

impl OutgoingPacket for GameEventPacket {
    fn encode_payload(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        self.event.write(writer)?;
        self.value.write(writer)?;
        Ok(())
    }
}
