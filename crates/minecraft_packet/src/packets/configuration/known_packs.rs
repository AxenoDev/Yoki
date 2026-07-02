use yoki_binutils::{BinaryError, BinaryReader, BinaryWriter, ReadBytes, WriteBytes};
use yoki_macros::{PacketIn, PacketOut};

#[derive(Debug, Clone)]
pub struct KnownPack {
    pub namespace: String,
    pub id: String,
    pub version: String,
}

impl ReadBytes for KnownPack {
    fn read(reader: &mut BinaryReader<'_>) -> Result<Self, BinaryError> {
        Ok(Self {
            namespace: String::read(reader)?,
            id: String::read(reader)?,
            version: String::read(reader)?,
        })
    }
}

impl WriteBytes for KnownPack {
    fn write(&self, writer: &mut BinaryWriter) -> Result<(), BinaryError> {
        self.namespace.write(writer)?;
        self.id.write(writer)?;
        self.version.write(writer)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PacketIn)]
pub struct SelectKnownPacksPacket {
    pub known_packs: Vec<KnownPack>,
}

#[derive(Debug, Clone, PacketOut)]
pub struct SelectKnownPacksClientboundPacket {
    pub known_packs: Vec<KnownPack>,
}
