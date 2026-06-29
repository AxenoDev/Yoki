use uuid::Uuid;

use crate::ProtocolError;
use crate::binary_reader::{BinaryReader, ReadBytes};
use crate::binary_writer::{BinaryWriter, WriteBytes};
use crate::data_types::VarInt;

pub trait ProtocolRead: Sized {
    fn read_from(reader: &mut BinaryReader<'_>) -> Result<Self, ProtocolError>;
}

pub trait ProtocolWrite {
    fn write_to(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError>;
}

macro_rules! impl_protocol_via_bytes {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl ProtocolRead for $ty {
                fn read_from(reader: &mut BinaryReader<'_>) -> Result<Self, ProtocolError> {
                    reader.read().map_err(Into::into)
                }
            }

            impl ProtocolWrite for $ty {
                fn write_to(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
                    self.write(writer).map_err(Into::into)
                }
            }
        )+
    };
}

impl_protocol_via_bytes!(bool, i8, u8, u16, i32, i64, f64, String, Uuid, VarInt,);

impl<T: ReadBytes> ProtocolRead for Vec<T> {
    fn read_from(reader: &mut BinaryReader<'_>) -> Result<Self, ProtocolError> {
        reader.read().map_err(Into::into)
    }
}

impl<T: WriteBytes> ProtocolWrite for Vec<T> {
    fn write_to(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        self.write(writer).map_err(Into::into)
    }
}

impl<T: ReadBytes> ProtocolRead for Option<T> {
    fn read_from(reader: &mut BinaryReader<'_>) -> Result<Self, ProtocolError> {
        reader.read().map_err(Into::into)
    }
}

impl<T: WriteBytes> ProtocolWrite for Option<T> {
    fn write_to(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        self.write(writer).map_err(Into::into)
    }
}
