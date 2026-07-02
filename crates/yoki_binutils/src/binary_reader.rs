use std::io::{Cursor, Read};

use crate::binary_error::BinaryError;

pub trait ReadBytes: Sized {
    fn read(reader: &mut BinaryReader<'_>) -> Result<Self, BinaryError>;
}

pub struct BinaryReader<'a>(Cursor<&'a [u8]>);

impl<'a> BinaryReader<'a> {
    pub fn new(raw: &'a [u8]) -> Self {
        Self(Cursor::new(raw))
    }

    pub fn read<T: ReadBytes>(&mut self) -> Result<T, BinaryError> {
        T::read(self)
    }

    pub(crate) fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), BinaryError> {
        self.0.read_exact(buf).map_err(BinaryError::from_io)
    }

    pub fn read_byte(&mut self) -> Result<u8, BinaryError> {
        let mut byte = [0u8; 1];
        self.read_exact(&mut byte)?;
        Ok(byte[0])
    }

    pub fn remaining(&self) -> usize {
        let total_len = self.0.get_ref().len();
        let current_pos = self.0.position() as usize;
        total_len.saturating_sub(current_pos)
    }

    pub fn remaining_bytes(&mut self) -> Result<Vec<u8>, BinaryError> {
        let mut buf = vec![0u8; self.remaining()];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }

    pub fn take_remaining_bytes(&mut self) -> Vec<u8> {
        self.remaining_bytes().unwrap_or_default()
    }

    pub fn read_byte_array(&mut self) -> Result<Vec<u8>, BinaryError> {
        self.read()
    }

    pub fn position(&self) -> u64 {
        self.0.position()
    }
}

pub(crate) fn read_be<const N: usize>(
    reader: &mut BinaryReader<'_>,
) -> Result<[u8; N], BinaryError> {
    let mut bytes = [0u8; N];
    reader.read_exact(&mut bytes)?;
    Ok(bytes)
}
