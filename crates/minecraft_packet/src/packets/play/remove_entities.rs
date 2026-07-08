use crate::packet::{OutgoingPacket, PacketDirection, PacketMeta};
use yoki_binutils::{BinaryWriter, ProtocolError, WriteBytes, data_types::VarInt};

#[derive(Debug, Clone)]
pub struct RemoveEntitiesPacket {
    entity_ids: Vec<VarInt>,
}

impl RemoveEntitiesPacket {
    pub fn new(entity_ids: impl IntoIterator<Item = i32>) -> Self {
        Self {
            entity_ids: entity_ids.into_iter().map(VarInt::new).collect(),
        }
    }
}

impl PacketMeta for RemoveEntitiesPacket {
    const DIRECTION: PacketDirection = PacketDirection::Out;
}

impl OutgoingPacket for RemoveEntitiesPacket {
    fn encode_payload(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        self.entity_ids.write(writer)?;
        Ok(())
    }
}
