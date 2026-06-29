use crate::ProtocolError;
use crate::binary_error::BinaryError;
use crate::binary_reader::{BinaryReader, ReadBytes};
use crate::binary_writer::{BinaryWriter, WriteBytes};
use crate::protocol::{ProtocolRead, ProtocolWrite};

#[derive(Clone, Debug)]
pub enum Omitted<T> {
    None,
    Some(T),
}

impl<T: ReadBytes> ReadBytes for Omitted<T> {
    fn read(reader: &mut BinaryReader<'_>) -> Result<Self, BinaryError> {
        if reader.remaining() == 0 {
            Ok(Self::None)
        } else {
            Ok(Self::Some(T::read(reader)?))
        }
    }
}

impl<T: WriteBytes> WriteBytes for Omitted<T> {
    fn write(&self, writer: &mut BinaryWriter) -> Result<(), BinaryError> {
        if let Self::Some(value) = self {
            value.write(writer)?;
        }
        Ok(())
    }
}

impl<T: WriteBytes> ProtocolWrite for Omitted<T> {
    fn write_to(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        self.write(writer).map_err(Into::into)
    }
}

impl<T: ReadBytes> ProtocolRead for Omitted<T> {
    fn read_from(reader: &mut BinaryReader<'_>) -> Result<Self, ProtocolError> {
        reader.read().map_err(Into::into)
    }
}
