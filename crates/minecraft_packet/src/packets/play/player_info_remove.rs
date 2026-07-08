use crate::packet::{OutgoingPacket, PacketDirection, PacketMeta};
use uuid::Uuid;
use yoki_binutils::{BinaryWriter, ProtocolError, WriteBytes};

#[derive(Debug, Clone)]
pub struct PlayerInfoRemovePacket {
    uuids: Vec<Uuid>,
}

impl PlayerInfoRemovePacket {
    pub fn new(uuids: Vec<Uuid>) -> Self {
        Self { uuids }
    }
}

impl PacketMeta for PlayerInfoRemovePacket {
    const DIRECTION: PacketDirection = PacketDirection::Out;
}

impl OutgoingPacket for PlayerInfoRemovePacket {
    fn encode_payload(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        self.uuids.write(writer)?;
        Ok(())
    }
}
