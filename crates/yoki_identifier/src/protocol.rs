use yoki_binutils::{
    BinaryError, BinaryReader, BinaryWriter, ProtocolError, ProtocolRead, ProtocolWrite, ReadBytes,
    WriteBytes,
};

use crate::Identifier;

impl ReadBytes for Identifier {
    fn read(reader: &mut BinaryReader<'_>) -> Result<Self, BinaryError> {
        let value = String::read(reader)?;
        Self::parse(&value).map_err(|err| BinaryError::InvalidIdentifier(err.to_string()))
    }
}

impl WriteBytes for Identifier {
    fn write(&self, writer: &mut BinaryWriter) -> Result<(), BinaryError> {
        self.to_string().write(writer)
    }
}

impl ProtocolWrite for Identifier {
    fn write_to(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        self.write(writer).map_err(Into::into)
    }
}

impl ProtocolRead for Identifier {
    fn read_from(reader: &mut BinaryReader<'_>) -> Result<Self, ProtocolError> {
        reader.read().map_err(Into::into)
    }
}
