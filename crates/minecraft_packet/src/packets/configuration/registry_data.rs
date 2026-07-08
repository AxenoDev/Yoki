use yoki_binutils::{
    BinaryError, BinaryWriter, ProtocolError, ProtocolWrite, WriteBytes, data_types::NbtTag,
};
use yoki_identifier::Identifier;
use yoki_macros::PacketOut;

#[derive(Debug, Clone)]
pub struct RegistryDataEntry {
    pub entry_id: Identifier,
    pub data: Option<NbtTag>,
}

impl WriteBytes for RegistryDataEntry {
    fn write(&self, writer: &mut BinaryWriter) -> Result<(), BinaryError> {
        self.entry_id.write(writer)?;
        self.data.write(writer)?;
        Ok(())
    }
}

impl ProtocolWrite for RegistryDataEntry {
    fn write_to(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        self.write(writer).map_err(Into::into)
    }
}

#[derive(Debug, Clone, PacketOut)]
pub struct RegistryDataPacket {
    pub registry_id: Identifier,
    pub entries: Vec<RegistryDataEntry>,
}

impl RegistryDataPacket {
    pub fn new(registry_id: Identifier, entries: Vec<RegistryDataEntry>) -> Self {
        Self {
            registry_id,
            entries,
        }
    }
}
