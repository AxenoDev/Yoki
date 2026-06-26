use uuid::Uuid;

use crate::error::ProtocolError;
use crate::reader::PacketReader;
use crate::writer::PacketWriter;

pub trait ProtocolRead: Sized {
    fn read_from(reader: &mut PacketReader<'_>) -> Result<Self, ProtocolError>;
}

pub trait ProtocolWrite {
    fn write_to(&self, writer: &mut PacketWriter) -> Result<(), ProtocolError>;
}

impl ProtocolRead for bool {
    fn read_from(reader: &mut PacketReader<'_>) -> Result<Self, ProtocolError> {
        reader.read_bool()
    }
}

impl ProtocolWrite for bool {
    fn write_to(&self, writer: &mut PacketWriter) -> Result<(), ProtocolError> {
        writer.write_bool(*self);
        Ok(())
    }
}

impl ProtocolRead for i8 {
    fn read_from(reader: &mut PacketReader<'_>) -> Result<Self, ProtocolError> {
        reader.read_i8()
    }
}

impl ProtocolWrite for i8 {
    fn write_to(&self, writer: &mut PacketWriter) -> Result<(), ProtocolError> {
        writer.write_i8(*self);
        Ok(())
    }
}

impl ProtocolRead for u8 {
    fn read_from(reader: &mut PacketReader<'_>) -> Result<Self, ProtocolError> {
        reader.read_u8()
    }
}

impl ProtocolWrite for u8 {
    fn write_to(&self, writer: &mut PacketWriter) -> Result<(), ProtocolError> {
        writer.write_u8(*self);
        Ok(())
    }
}

impl ProtocolRead for u16 {
    fn read_from(reader: &mut PacketReader<'_>) -> Result<Self, ProtocolError> {
        reader.read_u16()
    }
}

impl ProtocolWrite for u16 {
    fn write_to(&self, writer: &mut PacketWriter) -> Result<(), ProtocolError> {
        writer.write_u16(*self);
        Ok(())
    }
}

impl ProtocolRead for i32 {
    fn read_from(reader: &mut PacketReader<'_>) -> Result<Self, ProtocolError> {
        reader.read_varint()
    }
}

impl ProtocolWrite for i32 {
    fn write_to(&self, writer: &mut PacketWriter) -> Result<(), ProtocolError> {
        writer.write_varint(*self);
        Ok(())
    }
}

impl ProtocolRead for i64 {
    fn read_from(reader: &mut PacketReader<'_>) -> Result<Self, ProtocolError> {
        reader.read_i64()
    }
}

impl ProtocolWrite for i64 {
    fn write_to(&self, writer: &mut PacketWriter) -> Result<(), ProtocolError> {
        writer.write_i64(*self);
        Ok(())
    }
}

impl ProtocolRead for String {
    fn read_from(reader: &mut PacketReader<'_>) -> Result<Self, ProtocolError> {
        reader.read_string()
    }
}

impl ProtocolWrite for String {
    fn write_to(&self, writer: &mut PacketWriter) -> Result<(), ProtocolError> {
        writer.write_string(self);
        Ok(())
    }
}

impl ProtocolRead for Uuid {
    fn read_from(reader: &mut PacketReader<'_>) -> Result<Self, ProtocolError> {
        reader.read_uuid()
    }
}

impl ProtocolWrite for Uuid {
    fn write_to(&self, writer: &mut PacketWriter) -> Result<(), ProtocolError> {
        writer.write_uuid(self);
        Ok(())
    }
}

impl<T: ProtocolRead> ProtocolRead for Vec<T> {
    fn read_from(reader: &mut PacketReader<'_>) -> Result<Self, ProtocolError> {
        let len = reader.read_varint()? as usize;
        let mut items = Vec::with_capacity(len);
        for _ in 0..len {
            items.push(T::read_from(reader)?);
        }
        Ok(items)
    }
}

impl<T: ProtocolWrite> ProtocolWrite for Vec<T> {
    fn write_to(&self, writer: &mut PacketWriter) -> Result<(), ProtocolError> {
        writer.write_varint(self.len() as i32);
        for item in self {
            item.write_to(writer)?;
        }
        Ok(())
    }
}

impl<T: ProtocolRead> ProtocolRead for Option<T> {
    fn read_from(reader: &mut PacketReader<'_>) -> Result<Self, ProtocolError> {
        if reader.read_bool()? {
            Ok(Some(T::read_from(reader)?))
        } else {
            Ok(None)
        }
    }
}

impl<T: ProtocolWrite> ProtocolWrite for Option<T> {
    fn write_to(&self, writer: &mut PacketWriter) -> Result<(), ProtocolError> {
        match self {
            Some(value) => {
                writer.write_bool(true);
                value.write_to(writer)?;
            }
            None => writer.write_bool(false),
        }
        Ok(())
    }
}
