use crate::binary_error::BinaryError;
use crate::binary_reader::{read_be, BinaryReader, ReadBytes};
use crate::binary_writer::{write_be, BinaryWriter, WriteBytes};

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
