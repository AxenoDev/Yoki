use uuid::Uuid;

use crate::binary_error::BinaryError;
use crate::binary_reader::{read_be, BinaryReader, ReadBytes};
use crate::binary_writer::{BinaryWriter, WriteBytes};

impl ReadBytes for Uuid {
    fn read(reader: &mut BinaryReader<'_>) -> Result<Self, BinaryError> {
        let bytes = read_be::<16>(reader)?;
        Uuid::from_slice(&bytes).map_err(|_| BinaryError::InvalidUuid)
    }
}

impl WriteBytes for Uuid {
    fn write(&self, writer: &mut BinaryWriter) -> Result<(), BinaryError> {
        writer.write_all(self.as_bytes());
        Ok(())
    }
}
