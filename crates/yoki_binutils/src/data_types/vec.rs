use crate::binary_error::BinaryError;
use crate::binary_reader::{BinaryReader, ReadBytes};
use crate::binary_writer::{BinaryWriter, WriteBytes};
use crate::data_types::VarInt;

impl<T: ReadBytes> ReadBytes for Vec<T> {
    fn read(reader: &mut BinaryReader<'_>) -> Result<Self, BinaryError> {
        let len = VarInt::read(reader)?.inner() as usize;
        let mut items = Vec::with_capacity(len);
        for _ in 0..len {
            items.push(T::read(reader)?);
        }
        Ok(items)
    }
}

impl<T: WriteBytes> WriteBytes for Vec<T> {
    fn write(&self, writer: &mut BinaryWriter) -> Result<(), BinaryError> {
        VarInt::from(self.len() as i32).write(writer)?;
        for item in self {
            item.write(writer)?;
        }
        Ok(())
    }
}
