use crate::binary_error::BinaryError;
use crate::binary_reader::{BinaryReader, ReadBytes};
use crate::binary_writer::{BinaryWriter, WriteBytes};

impl ReadBytes for i8 {
    fn read(reader: &mut BinaryReader<'_>) -> Result<Self, BinaryError> {
        Ok(reader.read_byte()? as i8)
    }
}

impl WriteBytes for i8 {
    fn write(&self, writer: &mut BinaryWriter) -> Result<(), BinaryError> {
        writer.write_byte(*self as u8);
        Ok(())
    }
}
