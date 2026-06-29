use std::num::TryFromIntError;

use crate::binary_error::BinaryError;
use crate::binary_reader::{BinaryReader, ReadBytes};
use crate::binary_writer::{BinaryWriter, WriteBytes};

pub const SEGMENT_BITS: u8 = 0x7F;
pub const CONTINUE_BIT: u8 = 0x80;

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Hash, Ord)]
pub struct VarInt(i32);

impl ReadBytes for VarInt {
    #[inline]
    fn read(reader: &mut BinaryReader<'_>) -> Result<Self, BinaryError> {
        let mut num_read = 0;
        let mut result: u32 = 0;

        loop {
            let byte = reader.read_byte()?;
            let value = (byte & SEGMENT_BITS) as u32;
            result |= value << (7 * num_read);

            num_read += 1;
            if num_read > 5 {
                return Err(BinaryError::VarIntTooBig);
            }

            if byte & CONTINUE_BIT == 0 {
                break;
            }
        }

        Ok(Self(result as i32))
    }
}

impl WriteBytes for VarInt {
    fn write(&self, writer: &mut BinaryWriter) -> Result<(), BinaryError> {
        let mut value = self.0 as u32;

        while value >= 0x80 {
            writer.write_byte((value as u8) | CONTINUE_BIT);
            value >>= 7;
        }

        writer.write_byte(value as u8);
        Ok(())
    }
}

impl VarInt {
    pub fn new(value: i32) -> Self {
        Self(value)
    }

    pub fn inner(&self) -> i32 {
        self.0
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut writer = BinaryWriter::new();
        self.write(&mut writer).expect("varint write is infallible");
        writer.into_inner()
    }
}

impl From<i32> for VarInt {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<&i32> for VarInt {
    fn from(value: &i32) -> Self {
        Self::from(*value)
    }
}

impl From<u32> for VarInt {
    fn from(value: u32) -> Self {
        Self(value as i32)
    }
}

impl From<&u32> for VarInt {
    fn from(value: &u32) -> Self {
        Self::from(*value)
    }
}

impl TryFrom<i64> for VarInt {
    type Error = TryFromIntError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(Self::from(i32::try_from(value)?))
    }
}

impl TryFrom<usize> for VarInt {
    type Error = TryFromIntError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(Self::from(i32::try_from(value)?))
    }
}
