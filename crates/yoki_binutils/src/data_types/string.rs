use crate::binary_error::BinaryError;
use crate::binary_reader::{BinaryReader, ReadBytes};
use crate::binary_writer::{BinaryWriter, WriteBytes};
use crate::data_types::VarInt;

impl ReadBytes for String {
    fn read(reader: &mut BinaryReader<'_>) -> Result<Self, BinaryError> {
        let len = VarInt::read(reader)?.inner() as usize;
        if reader.remaining() < len {
            return Err(BinaryError::UnexpectedEof);
        }

        let mut buf = vec![0u8; len];
        reader.read_exact(&mut buf)?;
        String::from_utf8(buf).map_err(BinaryError::from)
    }
}

impl WriteBytes for String {
    fn write(&self, writer: &mut BinaryWriter) -> Result<(), BinaryError> {
        VarInt::from(self.len() as i32).write(writer)?;
        writer.write_all(self.as_bytes());
        Ok(())
    }
}
