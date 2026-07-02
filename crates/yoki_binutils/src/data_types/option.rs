use crate::binary_error::BinaryError;
use crate::binary_reader::{BinaryReader, ReadBytes};
use crate::binary_writer::{BinaryWriter, WriteBytes};

impl<T: ReadBytes> ReadBytes for Option<T> {
    fn read(reader: &mut BinaryReader<'_>) -> Result<Self, BinaryError> {
        if bool::read(reader)? {
            Ok(Some(T::read(reader)?))
        } else {
            Ok(None)
        }
    }
}

impl<T: WriteBytes> WriteBytes for Option<T> {
    fn write(&self, writer: &mut BinaryWriter) -> Result<(), BinaryError> {
        match self {
            Some(value) => {
                true.write(writer)?;
                value.write(writer)?;
            }
            None => {
                false.write(writer)?;
            }
        }
        Ok(())
    }
}
