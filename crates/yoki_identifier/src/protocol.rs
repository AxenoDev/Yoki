use yoki_binutils::{
    ProtocolError, ProtocolRead, ProtocolWrite, reader::PacketReader, writer::PacketWriter,
};

use crate::Identifier;

impl ProtocolWrite for Identifier {
    fn write_to(&self, writer: &mut PacketWriter) -> Result<(), ProtocolError> {
        writer.write_string(&self.to_string());
        Ok(())
    }
}

impl ProtocolRead for Identifier {
    fn read_from(reader: &mut PacketReader<'_>) -> Result<Self, ProtocolError> {
        let value = reader.read_string()?;
        Self::parse(&value).map_err(|err| ProtocolError::InvalidIdentifier(err.to_string()))
    }
}
