use crate::packet::{IncomingPacket, OutgoingPacket, PacketDirection, PacketMeta};
use yoki_binutils::{
    BinaryReader, BinaryWriter, ProtocolError, ReadBytes, WriteBytes,
};

#[derive(Debug, Clone, Copy)]
pub struct ServerboundKeepAlivePacket {
    pub id: i64,
}

impl PacketMeta for ServerboundKeepAlivePacket {
    const DIRECTION: PacketDirection = PacketDirection::In;
}

impl IncomingPacket for ServerboundKeepAlivePacket {
    fn decode_payload(reader: &mut BinaryReader<'_>) -> Result<Self, ProtocolError> {
        Ok(Self { id: i64::read(reader)? })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ClientboundKeepAlivePacket {
    pub id: i64,
}

impl ClientboundKeepAlivePacket {
    pub const fn new(id: i64) -> Self {
        Self { id }
    }
}

impl PacketMeta for ClientboundKeepAlivePacket {
    const DIRECTION: PacketDirection = PacketDirection::Out;
}

impl OutgoingPacket for ClientboundKeepAlivePacket {
    fn encode_payload(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        self.id.write(writer)?;
        Ok(())
    }
}
