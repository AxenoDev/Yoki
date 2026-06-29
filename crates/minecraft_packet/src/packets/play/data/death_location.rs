use yoki_binutils::{
    BinaryError, BinaryReader, BinaryWriter, ProtocolError, ProtocolRead, ProtocolWrite,
    ReadBytes, WriteBytes, data_types::Omitted,
};
use yoki_identifier::Identifier;

use crate::packets::play::data::Position;

#[derive(Clone, Debug)]
pub struct DeathLocation {
    dimension: Omitted<Identifier>,
    location: Omitted<Position>,
}

impl ReadBytes for DeathLocation {
    fn read(reader: &mut BinaryReader<'_>) -> Result<Self, BinaryError> {
        Ok(Self {
            dimension: Omitted::read(reader)?,
            location: Omitted::read(reader)?,
        })
    }
}

impl WriteBytes for DeathLocation {
    fn write(&self, writer: &mut BinaryWriter) -> Result<(), BinaryError> {
        self.dimension.write(writer)?;
        self.location.write(writer)?;
        Ok(())
    }
}

impl ProtocolWrite for DeathLocation {
    fn write_to(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        self.write(writer).map_err(Into::into)
    }
}

impl ProtocolRead for DeathLocation {
    fn read_from(reader: &mut BinaryReader<'_>) -> Result<Self, ProtocolError> {
        reader.read().map_err(Into::into)
    }
}
