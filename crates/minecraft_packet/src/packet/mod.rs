mod raw;

pub use raw::RawPacket;
use yoki_binutils::{
    BinaryReader, BinaryWriter, ProtocolError, WriteBytes, data_types::VarInt,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketDirection {
    In,
    Out,
}

pub trait PacketMeta {
    const DIRECTION: PacketDirection;
}

pub trait IncomingPacket: PacketMeta {
    fn decode_payload(reader: &mut BinaryReader<'_>) -> Result<Self, ProtocolError>
    where
        Self: Sized;
}

pub trait OutgoingPacket: PacketMeta {
    fn encode_payload(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError>;

    fn encode_with_id(&self, id: i32) -> Result<Vec<u8>, ProtocolError> {
        let mut writer = BinaryWriter::new();
        VarInt::from(id).write(&mut writer)?;
        self.encode_payload(&mut writer)?;
        Ok(writer.into_inner())
    }
}
