use crate::binary_error::BinaryError;
use crate::binary_reader::{BinaryReader, ReadBytes, read_be};
use crate::binary_writer::{BinaryWriter, WriteBytes, write_be};

impl ReadBytes for f32 {
    fn read(reader: &mut BinaryReader<'_>) -> Result<Self, BinaryError> {
        Ok(f32::from_be_bytes(read_be::<4>(reader)?))
    }
}

impl WriteBytes for f32 {
    fn write(&self, writer: &mut BinaryWriter) -> Result<(), BinaryError> {
        write_be(writer, self.to_be_bytes());
        Ok(())
    }
}
