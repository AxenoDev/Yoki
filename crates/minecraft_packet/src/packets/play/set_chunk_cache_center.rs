use crate::packet::{OutgoingPacket, PacketDirection, PacketMeta};
use yoki_binutils::{
    BinaryWriter, ProtocolError, WriteBytes,
    data_types::VarInt,
};

#[derive(Debug, Clone)]
pub struct SetChunkCacheCenterPacket {
    chunk_x: VarInt,
    chunk_z: VarInt,
}

impl SetChunkCacheCenterPacket {
    pub fn new(chunk_x: i32, chunk_z: i32) -> Self {
        Self {
            chunk_x: VarInt::new(chunk_x),
            chunk_z: VarInt::new(chunk_z),
        }
    }
}

impl PacketMeta for SetChunkCacheCenterPacket {
    const DIRECTION: PacketDirection = PacketDirection::Out;
}

impl OutgoingPacket for SetChunkCacheCenterPacket {
    fn encode_payload(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        self.chunk_x.write(writer)?;
        self.chunk_z.write(writer)?;
        Ok(())
    }
}
