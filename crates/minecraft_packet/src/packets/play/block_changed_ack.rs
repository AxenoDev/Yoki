use crate::packet::{OutgoingPacket, PacketDirection, PacketMeta};
use yoki_binutils::{data_types::VarInt, BinaryWriter, ProtocolError, WriteBytes};

#[derive(Debug, Clone)]
pub struct BlockChangedAckPacket {
    sequence: VarInt,
}

impl BlockChangedAckPacket {
    pub fn new(sequence: i32) -> Self {
        Self {
            sequence: VarInt::new(sequence),
        }
    }
}

impl PacketMeta for BlockChangedAckPacket {
    const DIRECTION: PacketDirection = PacketDirection::Out;
}

impl OutgoingPacket for BlockChangedAckPacket {
    fn encode_payload(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        self.sequence.write(writer)?;
        Ok(())
    }
}
