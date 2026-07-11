use crate::packet::{OutgoingPacket, PacketDirection, PacketMeta};
use crate::packets::play::data::BlockPos;
use yoki_binutils::{data_types::VarInt, BinaryWriter, ProtocolError, WriteBytes};

#[derive(Debug, Clone)]
pub struct BlockUpdatePacket {
    pos: BlockPos,
    state: VarInt,
}

impl BlockUpdatePacket {
    pub fn new(pos: BlockPos, block_state_id: i32) -> Self {
        Self {
            pos,
            state: VarInt::new(block_state_id),
        }
    }
}

impl PacketMeta for BlockUpdatePacket {
    const DIRECTION: PacketDirection = PacketDirection::Out;
}

impl OutgoingPacket for BlockUpdatePacket {
    fn encode_payload(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        self.pos.write(writer)?;
        self.state.write(writer)?;
        Ok(())
    }
}
