mod raw;

pub use raw::RawPacket;
use yoki_binutils::{ProtocolError, reader::PacketReader, writer::PacketWriter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketDirection {
    In,
    Out,
}

pub trait PacketMeta {
    const ID: i32;
    const DIRECTION: PacketDirection;
}

pub trait IncomingPacket: PacketMeta {
    fn decode_payload(reader: &mut PacketReader<'_>) -> Result<Self, ProtocolError>
    where
        Self: Sized;
}

pub trait OutgoingPacket: PacketMeta {
    fn encode_payload(&self, writer: &mut PacketWriter) -> Result<(), ProtocolError>;

    fn encode(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut writer = PacketWriter::new();
        writer.write_varint(Self::ID);
        self.encode_payload(&mut writer)?;
        Ok(writer.into_inner())
    }
}
