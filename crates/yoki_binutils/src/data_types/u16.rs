use crate::binary_error::BinaryError;
use crate::binary_reader::{BinaryReader, ReadBytes, read_be};
use crate::binary_writer::{BinaryWriter, WriteBytes, write_be};

impl ReadBytes for u16 {
    fn read(reader: &mut BinaryReader<'_>) -> Result<Self, BinaryError> {
        Ok(u16::from_be_bytes(read_be::<2>(reader)?))
    }
}

impl WriteBytes for u16 {
    fn write(&self, writer: &mut BinaryWriter) -> Result<(), BinaryError> {
        write_be(writer, self.to_be_bytes());
        Ok(())
    }
}
