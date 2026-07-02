use crate::binary_error::BinaryError;
use crate::binary_reader::{BinaryReader, ReadBytes, read_be};
use crate::binary_writer::{BinaryWriter, WriteBytes, write_be};

impl ReadBytes for i32 {
    fn read(reader: &mut BinaryReader<'_>) -> Result<Self, BinaryError> {
        Ok(i32::from_be_bytes(read_be::<4>(reader)?))
    }
}

impl WriteBytes for i32 {
    fn write(&self, writer: &mut BinaryWriter) -> Result<(), BinaryError> {
        write_be(writer, self.to_be_bytes());
        Ok(())
    }
}
