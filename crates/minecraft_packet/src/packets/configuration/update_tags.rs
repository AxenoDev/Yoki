use yoki_binutils::{
    BinaryError, BinaryReader, BinaryWriter, ProtocolError, ProtocolWrite, ReadBytes, WriteBytes,
    data_types::VarInt,
};
use yoki_identifier::Identifier;
use yoki_macros::PacketOut;

#[derive(Debug, Clone)]
pub struct TagEntry {
    pub name: Identifier,
    pub entries: Vec<VarInt>,
}

#[derive(Debug, Clone)]
pub struct RegistryTags {
    pub registry: Identifier,
    pub tags: Vec<TagEntry>,
}

impl ProtocolWrite for TagEntry {
    fn write_to(&self, writer: &mut yoki_binutils::BinaryWriter) -> Result<(), ProtocolError> {
        self.name.write_to(writer)?;
        self.entries.write_to(writer)?;
        Ok(())
    }
}

impl WriteBytes for TagEntry {
    fn write(&self, writer: &mut BinaryWriter) -> Result<(), BinaryError> {
        self.name.write(writer)?;
        self.entries.write(writer)?;
        Ok(())
    }
}

impl ProtocolWrite for RegistryTags {
    fn write_to(&self, writer: &mut yoki_binutils::BinaryWriter) -> Result<(), ProtocolError> {
        self.registry.write_to(writer)?;
        self.tags.write_to(writer)?;
        Ok(())
    }
}

impl WriteBytes for RegistryTags {
    fn write(&self, writer: &mut BinaryWriter) -> Result<(), BinaryError> {
        self.registry.write(writer)?;
        self.tags.write(writer)?;
        Ok(())
    }
}

impl ReadBytes for TagEntry {
    fn read(reader: &mut BinaryReader<'_>) -> Result<Self, BinaryError> {
        Ok(Self {
            name: Identifier::read(reader)?,
            entries: Vec::<VarInt>::read(reader)?,
        })
    }
}

impl ReadBytes for RegistryTags {
    fn read(reader: &mut BinaryReader<'_>) -> Result<Self, BinaryError> {
        Ok(Self {
            registry: Identifier::read(reader)?,
            tags: Vec::<TagEntry>::read(reader)?,
        })
    }
}

#[derive(Debug, Clone, PacketOut)]
pub struct UpdateTagsPacket {
    pub registries: Vec<RegistryTags>,
}
