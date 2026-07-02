use crate::binary_error::BinaryError;
use crate::binary_reader::{BinaryReader, ReadBytes};
use crate::binary_writer::{BinaryWriter, WriteBytes};

impl ReadBytes for u8 {
    fn read(reader: &mut BinaryReader<'_>) -> Result<Self, BinaryError> {
        reader.read_byte()
    }
}

impl WriteBytes for u8 {
    fn write(&self, writer: &mut BinaryWriter) -> Result<(), BinaryError> {
        writer.write_byte(*self);
        Ok(())
    }
}
