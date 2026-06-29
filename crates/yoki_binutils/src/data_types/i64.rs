use crate::binary_error::BinaryError;
use crate::binary_reader::{BinaryReader, ReadBytes, read_be};
use crate::binary_writer::{BinaryWriter, WriteBytes, write_be};

impl ReadBytes for i64 {
    fn read(reader: &mut BinaryReader<'_>) -> Result<Self, BinaryError> {
        Ok(i64::from_be_bytes(read_be::<8>(reader)?))
    }
}

impl WriteBytes for i64 {
    fn write(&self, writer: &mut BinaryWriter) -> Result<(), BinaryError> {
        write_be(writer, self.to_be_bytes());
        Ok(())
    }
}
